use actix_web::{client, rt, web, App, HttpServer, Result};
use image::{self};
mod ruxel;
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::sync::Mutex;
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
                .app_data(State {
                    canvas: Mutex::new(image::ImageBuffer::new(0, 0)),
                })
                .configure(|cfg: &mut web::ServiceConfig| config_clone.proxy_config(cfg))
                .route("/print", web::post().to(print_handler))
                .route("/exit", web::post().to(exit_handler))
                .route("/load", web::post().to(load_handler))
                .route("/save", web::post().to(save_handler))
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

struct State {
    canvas: Mutex<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
}

async fn print_handler(bytes: web::Bytes) -> Result<web::Bytes> {
    print!("{}", std::str::from_utf8(&bytes[..]).unwrap());
    io::stdout().flush().unwrap();

    Ok(web::Bytes::new())
}
async fn exit_handler() -> Result<web::Bytes> {
    async { exit(0) }.await;
    Ok(web::Bytes::new())
}

async fn save_handler(state: web::Data<State>, bytes: web::Bytes) -> Result<web::Bytes> {
    let file_name = std::str::from_utf8(&bytes[..]).unwrap();
    let canvas = state.canvas.lock().unwrap();
    canvas.save(file_name).unwrap();
    Ok(web::Bytes::new())
}
async fn load_handler(state: web::Data<State>, bytes: web::Bytes) -> Result<web::Bytes> {
    let mut canvas = state.canvas.lock().unwrap();
    let file_name = std::str::from_utf8(&bytes[..]).unwrap();
    let img = image::io::Reader::open(file_name)?.decode().unwrap();
    let img_buf = img.as_rgba8().unwrap().clone();
    *canvas = img_buf;
    Ok(web::Bytes::new())
}
