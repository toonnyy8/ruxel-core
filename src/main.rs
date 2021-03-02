mod ruxel;
use actix_web::{client, rt, web, App, HttpServer, Result};
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::thread;

#[actix_web::main]
async fn main() {
    let config = ruxel::Config::import("ruxel.json");
    let core_port = config.core_port();
    config.start();

    let config_clone = config.clone();
    thread::spawn(move || {
        let sys = rt::System::new("http-server");

        HttpServer::new(move || {
            App::new()
                .configure(|cfg: &mut web::ServiceConfig| config_clone.proxy_config(cfg))
                .route("/print", web::post().to(print_handler))
                .route("/exit", web::post().to(exit_handler))
        })
        .bind(format!("127.0.0.1:{}", core_port))?
        .run();
        sys.run()
    });

    let config_clone = config.clone();
    let http_client = client::Client::new();
    for line in std::io::stdin().lock().lines() {
        let cmd = line.unwrap();
        config_clone.run(&http_client, cmd.as_str()).await;
    }
}

async fn print_handler(bytes: web::Bytes) -> Result<web::Bytes> {
    print!("{}", std::str::from_utf8(&bytes[..]).unwrap());
    io::stdout().flush().unwrap();

    Ok(web::Bytes::new())
}
async fn exit_handler() -> Result<web::Bytes> {
    exit(0);
    Ok(web::Bytes::new())
}
