use actix_web::{client, dev::Body, web, Result};
use serde_json;
use std::process::Command;
use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
pub struct Config {
    core_port: u16,
    addons: Vec<Addon>,
    proxies: Vec<Proxy>,
    methods: Vec<Method>,
}
impl Config {
    pub fn import(path: &str) -> Self {
        let config_file = fs::read_to_string(path).expect("Something went wrong reading the file");
        let config_json: serde_json::Value = serde_json::from_str(&config_file).unwrap();

        let core_port = config_json.get("core_port").unwrap().as_i64().unwrap() as u16;

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

        let methods = config_json
            .get("methods")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|method| {
                let reg = method.get("regex").unwrap().as_str().unwrap();
                let reg = regex::Regex::from_str(reg).unwrap();
                let run = method
                    .get("run")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|app| {
                        let addon_name = app.get("addon").unwrap().as_str().unwrap();
                        let addon_idx = addons
                            .iter()
                            .position(|addon| addon.name == addon_name)
                            .unwrap();
                        let route = String::from(app.get("route").unwrap().as_str().unwrap());
                        App { addon_idx, route }
                    })
                    .collect::<Vec<_>>();
                Method { reg, run }
            })
            .collect::<Vec<_>>();

        return Self {
            core_port,
            addons,
            proxies,
            methods,
        };
    }

    pub fn start(&self) {
        let core_port = self.core_port;
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
                        if addon_name == "core" {
                            format!("{}", core_port)
                        } else {
                            format!(
                                "{}",
                                addons
                                    .iter()
                                    .find(|_addon| { _addon.name == addon_name })
                                    .unwrap()
                                    .port
                            )
                        }
                    } else {
                        arg.clone()
                    }
                })
                .collect::<Vec<_>>();
            let expect = &addon.start.expect;
            Command::new(cmd)
                .current_dir(dir)
                .arg("run")
                .args(args)
                .spawn()
                .expect(expect.as_str());
        }
    }

    pub async fn run(&self, http_client: &client::Client, cmd: &str) {
        let method = self
            .methods
            .iter()
            .find(|method| method.reg.is_match(cmd))
            .unwrap();
        for app in &method.run {
            let url = format!(
                "http://localhost:{}/{}",
                self.addons[app.addon_idx].port, app.route
            );

            http_client
                .post(url.as_str())
                .send_body(Body::from_message(String::from(cmd)))
                .await
                .unwrap();
        }
    }

    pub fn core_port(&self) -> u16 {
        self.core_port
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
async fn proxy_hanbler(bytes: web::Bytes, proxy: web::Data<ProxyUrl>) -> Result<web::Bytes> {
    let http_client = client::Client::new();
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
    run: Vec<App>,
}

#[derive(Debug, Clone)]
struct App {
    addon_idx: usize,
    route: String,
}
