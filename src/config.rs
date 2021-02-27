use bytes;
use regex;
use reqwest;
use serde_json;
use std::boxed::Box;
use std::process::Command;
use std::{fs, str::FromStr};
use warp::{http::response, Filter};

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

    pub fn run(&self, client: &reqwest::blocking::Client, cmd: &str) {
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
            client
                .post(url.as_str())
                .body(String::from(cmd))
                .send()
                .unwrap();
        }
    }

    pub fn core_port(&self) -> u16 {
        self.core_port
    }

    // pub fn build_proxies(
    //     &'static self,
    // ) -> Result<impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone, &str>
    // {
    //     let mut proxy_post1 = warp::post()
    //         .and(warp::path("cmd"))
    //         .and(warp::any().map(move || reqwest::Client::new()))
    //         .and(warp::body::bytes())
    //         .and_then(
    //             |http_client: reqwest::Client, bytes: bytes::Bytes| async move {
    //                 let cmd = std::str::from_utf8(&bytes[..]).unwrap();
    //                 let is_ok = http_client
    //                     .post("http://localhost:3030/print")
    //                     .body(format!("{}\n:", cmd))
    //                     .send()
    //                     .await;
    //                 match is_ok {
    //                     Ok(_) => Ok(""),
    //                     Err(_) => Err(warp::reject::not_found()),
    //                 }
    //             },
    //         );
    //     // let mut proxy_post2;
    //     for idx in 0..self.proxies.len() {
    //         let proxy = &self.proxies[idx];
    //         let proxy_url = format!("http://localhost:{}/{}", self.core_port, &proxy.proxy_route);
    //         let _proxy_post = warp::post()
    //             .and(warp::path(proxy_url))
    //             .and(warp::any().map(move || reqwest::Client::new()))
    //             .and(warp::any().map(move || proxy.clone()))
    //             .and(warp::body::bytes())
    //             .and_then(
    //                 |http_client: reqwest::Client, proxy: Proxy, bytes: bytes::Bytes| async move {
    //                     let target_url = format!(
    //                         "http://localhost:{}/{}",
    //                         self.addons[proxy.addon_idx].port, proxy.route
    //                     );
    //                     let is_ok = http_client
    //                         .post(target_url.as_str())
    //                         .body(bytes)
    //                         .send()
    //                         .await;

    //                     match is_ok {
    //                         Ok(is_ok) => match is_ok.bytes().await {
    //                             Ok(ret) => Ok(warp::http::Response::builder().body(ret)),
    //                             Err(_) => Err(warp::reject::not_found()),
    //                         },
    //                         Err(_) => Err(warp::reject::not_found()),
    //                     }
    //                 },
    //             );
    //         // proxy_post1 = _proxy_post.or(_proxy_post);
    //         proxy_post1 = _proxy_post;
    //         // if idx == 0 {
    //         // proxy_post1 = _proxy_post;
    //         // } else {
    //         //     proxy_post1 = _proxy_post.or(_proxy_post);
    //         // }
    //     }
    //     if true {
    //         Ok(proxy_post1)
    //     } else {
    //         Err("")
    //     }

    //     // proxy_post1.or(proxy_post1)
    // }
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
// impl Proxy {
//     fn build(
//         &'static self,
//         core_port: u16,
//         addon_port: u16,
//     ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//         let proxy_url = format!("http://localhost:{}/{}", core_port, self.proxy_route);

//         warp::post()
//             .and(warp::path(proxy_url))
//             .and(warp::any().map(move || self.clone()))
//             .and(warp::any().map(move || reqwest::Client::new()))
//             .and(warp::body::bytes())
//             .and_then(
//                 |proxy: Proxy, http_client: reqwest::Client, bytes: bytes::Bytes| async move {
//                     // &self;
//                     proxy;
//                     // let target_url = format!("http://localhost:{}/{}", addon_port, self.route);
//                     // let is_ok = http_client
//                     //     .post(target_url.as_str())
//                     //     .body(bytes)
//                     //     .send()
//                     //     .await;
//                     // match is_ok {
//                     //     Ok(is_ok) => match is_ok.bytes().await {
//                     //         Ok(ret) => Ok(warp::reply()),
//                     //         Err(_) => Err(warp::reject::not_found()),
//                     //     },
//                     //     Err(_) => Err(warp::reject::not_found()),
//                     // }
//                     if true {
//                         Ok(warp::reply())
//                     } else {
//                         Err(warp::reject::not_found())
//                     }
//                 },
//             )
//     }
// }

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
