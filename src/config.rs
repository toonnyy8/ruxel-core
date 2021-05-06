use actix_web::{client, dev::Body, web, Result};
use regex;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Config {
    addons: Vec<Addon>,
    proxies: Vec<Proxy>,
    // methods: Vec<Method>,
    modes: HashMap<String, Vec<Act>>,
}
impl Config {
    pub fn import(path: &str) -> Self {
        let config_file = fs::read_to_string(path).expect("Something went wrong reading the file");
        let config_json: serde_json::Value = serde_json::from_str(&config_file).unwrap();

        let addons = config_json
            .get("addons")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|addon| {
                let name = String::from(addon.get("name").unwrap().as_str().unwrap());
                let port = addon.get("port").unwrap().as_i64().unwrap() as u16;
                let start_info = addon.get("start").unwrap();
                let start_cmd = String::from(start_info.get("cmd").unwrap().as_str().unwrap());
                let start_dir = String::from(start_info.get("dir").unwrap().as_str().unwrap());
                let args = start_info
                    .get("args")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|arg| String::from(arg.as_str().unwrap()))
                    .collect::<Vec<_>>();
                let start_expect =
                    String::from(start_info.get("expect").unwrap().as_str().unwrap());
                Addon {
                    name,
                    port,
                    start: Start {
                        cmd: start_cmd,
                        dir: start_dir,
                        args,
                        expect: start_expect,
                    },
                }
            })
            .collect::<Vec<_>>();

        let proxies = config_json
            .get("proxies")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|proxy| {
                let addon_name = proxy.get("addon").unwrap().as_str().unwrap();
                let addon_idx = addons
                    .iter()
                    .position(|addon| addon.name == addon_name)
                    .unwrap();
                let route = String::from(proxy.get("route").unwrap().as_str().unwrap());
                let proxy_route = String::from(proxy.get("proxy_route").unwrap().as_str().unwrap());

                Proxy {
                    addon_idx,
                    route,
                    proxy_route,
                }
            })
            .collect::<Vec<_>>();

        let modes = config_json
            .get("mode")
            .unwrap()
            .as_object()
            .unwrap()
            .iter()
            .fold(
                HashMap::<String, Vec<Act>>::new(),
                |mut prev, (mode_name, mode_act)| {
                    let acts = mode_act
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|act| {
                            let addon_name = act.get("addon").unwrap().as_str().unwrap();
                            let port = addons
                                .iter()
                                .find(|addon| addon.name == addon_name)
                                .unwrap()
                                .port;
                            let route = String::from(act.get("route").unwrap().as_str().unwrap());
                            Act { port, route }
                        })
                        .collect::<Vec<_>>();
                    prev.insert(mode_name.clone(), acts);
                    prev
                },
            );

        return Self {
            addons,
            proxies,
            modes,
        };
    }

    pub fn start(&self) {
        let addons = &self.addons;
        for addon in addons {
            let cmd = &addon.start.cmd;
            let dir = &addon.start.dir;

            let args = addon
                .start
                .args
                .iter()
                .map(|arg| {
                    if arg.len() > 5 && &arg[arg.len() - 5..] == ".port" {
                        let addon_name = &arg[..arg.len() - 5];
                        let port = addons
                            .iter()
                            .find(|_addon| _addon.name == addon_name)
                            .unwrap()
                            .port;
                        format!("{}", port)
                    } else {
                        arg.clone()
                    }
                })
                .collect::<Vec<_>>();
            let expect = &addon.start.expect;

            if cmd != "" {
                Command::new(cmd)
                    .current_dir(dir)
                    .args(args)
                    .spawn()
                    .expect(expect.as_str());
            }
        }
    }

    pub async fn run(&self, http_client: &client::Client, mode: &str, param: &str) {
        let acts = match self.modes.get(mode) {
            Some(acts) => acts.clone(),
            None => {
                let default_acts = match self.modes.get("default") {
                    Some(default_acts) => default_acts.clone(),
                    None => Vec::new(),
                };
                default_acts
            }
        };
        for act in acts {
            http_client
                .post(act.as_url())
                .send_body(Body::from_message(String::from(param)))
                .await
                .err();
        }
    }

    pub fn core_port(&self) -> u16 {
        self.addons
            .iter()
            .find(|addon| addon.name == "core")
            .unwrap()
            .port
    }

    pub fn proxy_config(&self, cfg: &mut web::ServiceConfig) {
        for proxy in &self.proxies {
            let target_url = format!(
                "http://localhost:{}/{}",
                self.addons[proxy.addon_idx].port, proxy.route
            );
            cfg.service(
                web::scope(proxy.proxy_route.as_str())
                    .data(ProxyUrl { url: target_url })
                    .route("", web::post().to(proxy_hanbler)),
            );
        }
    }
}

struct ProxyUrl {
    url: String,
}
async fn proxy_hanbler(
    http_client: web::Data<client::Client>,
    proxy: web::Data<ProxyUrl>,
    bytes: web::Bytes,
) -> Result<web::Bytes> {
    let ret = http_client
        .post(proxy.url.as_str())
        .send_body(bytes)
        .await
        .unwrap()
        .body()
        .await
        .unwrap();
    Ok(ret)
}

#[derive(Debug, Clone)]
struct Addon {
    name: String,
    port: u16,
    start: Start,
}
#[derive(Debug, Clone)]
struct Start {
    cmd: String,
    dir: String,
    args: Vec<String>,
    expect: String,
}

#[derive(Debug, Clone)]
struct Proxy {
    pub addon_idx: usize,
    pub route: String,
    pub proxy_route: String,
}

#[derive(Debug, Clone)]
struct Method {
    reg: regex::Regex,
    run: Vec<Act>,
}

#[derive(Debug, Clone)]
struct Act {
    port: u16,
    route: String,
}
impl Act {
    fn as_url(&self) -> String {
        format!("http://localhost:{}/{}", self.port, self.route)
    }
}
