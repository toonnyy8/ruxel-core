use actix_web::{client, dev::Body, rt, web, App, HttpServer, Result};
use image::{self, GenericImageView};
use serde_json;
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

    thread::spawn(move || {
        let sys = rt::System::new("http-server");
        let state = web::Data::new(ruxel::State {
            canvas: Mutex::new(image::ImageBuffer::new(0, 0)),
            mode: Mutex::new("default".to_string()),
        });

        HttpServer::new(move || {
            App::new()
                .data(client::Client::new())
                .data(config.clone())
                .app_data(state.clone())
                .configure(|cfg: &mut web::ServiceConfig| config.proxy_config(cfg))
                .route("/print", web::post().to(print_handler))
                .route("/exit", web::post().to(exit_handler))
                .route("/load", web::post().to(load_handler))
                .route("/save", web::post().to(save_handler))
                .route("/cmd", web::post().to(cmd_handler))
                .route("/get_canvas", web::post().to(get_canvas_handler))
                .route("/get_mode", web::post().to(get_mode_handler))
                .route("/canvas_info", web::post().to(canvas_info_handler))
        })
        .bind(format!("127.0.0.1:{}", core_port))?
        .run();

        sys.run()
    });

    let http_client = client::Client::new();
    for line in std::io::stdin().lock().lines() {
        let cmd = line.unwrap();
        http_client
            .post(format!("http://localhost:{}/cmd", core_port))
            .send_body(Body::from_message(String::from(cmd)))
            .await
            .err();
    }

    thread::park();
}

async fn cmd_handler(
    config: web::Data<ruxel::Config>,
    http_client: web::Data<client::Client>,
    state: web::Data<ruxel::State>,
    bytes: web::Bytes,
) -> Result<web::Bytes> {
    let cmd = std::str::from_utf8(&bytes[..]).unwrap();
    let tail_space_reg = regex::Regex::new(r"^(([\s]*[^\s])*)([\s]*)$").unwrap();
    let cmd = &tail_space_reg.captures(cmd).unwrap()[1];

    let change_mode_reg = regex::Regex::new(r"^:([^\s]+)$").unwrap();
    let eager_mode_reg = regex::Regex::new(r"^-([^\s]+)([\s]*)(.*)$").unwrap();
    let (mode, param) = if change_mode_reg.is_match(cmd) {
        let mode = &change_mode_reg.captures(cmd).unwrap()[1];
        let mut now_mode = state.mode.lock().unwrap();
        *now_mode = mode.to_string();

        ("default".to_string(), "".to_string())
    } else if eager_mode_reg.is_match(cmd) {
        let eager_mode_cap = eager_mode_reg.captures(cmd).unwrap();
        let mode = &eager_mode_cap[1];
        let param = &eager_mode_cap[3];

        (mode.to_string(), param.to_string())
    } else {
        let mode = state.mode.lock().unwrap();
        let param = cmd;

        (mode.clone(), param.to_string())
    };
    config
        .run(&http_client, mode.as_str(), param.as_str())
        .await;
    Ok(web::Bytes::new())
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

async fn save_handler(state: web::Data<ruxel::State>, bytes: web::Bytes) -> Result<web::Bytes> {
    let file_name = std::str::from_utf8(&bytes[..]).unwrap();
    let canvas = state.canvas.lock().unwrap();
    canvas.save(file_name).unwrap();
    Ok(web::Bytes::new())
}
async fn load_handler(state: web::Data<ruxel::State>, bytes: web::Bytes) -> Result<web::Bytes> {
    let mut canvas = state.canvas.lock().unwrap();
    let file_name = std::str::from_utf8(&bytes[..]).unwrap();
    let img = image::io::Reader::open(file_name)?.decode().unwrap();
    let img_buf = img.as_rgba8().unwrap().clone();
    *canvas = img_buf;
    Ok(web::Bytes::new())
}
async fn get_canvas_handler(
    state: web::Data<ruxel::State>,
    bytes: web::Bytes,
) -> Result<web::Bytes> {
    let canvas = state.canvas.lock().unwrap();
    let info: serde_json::Value = serde_json::from_slice(&bytes.to_vec()[..]).unwrap();
    let info = info.as_object().unwrap();
    fn get_u32(opt_val: &Option<&serde_json::Value>, init: u32) -> u32 {
        match opt_val {
            Some(val) => match val.as_i64() {
                Some(val) => val as u32,
                None => init,
            },
            None => init,
        }
    }
    let x = get_u32(&info.get("x"), 0);
    let y = get_u32(&info.get("y"), 0);
    let width = get_u32(&info.get("width"), canvas.width());
    let height = get_u32(&info.get("height"), canvas.height());
    Ok(web::Bytes::from(
        canvas.view(x, y, width, height).to_image().to_vec(),
    ))
}
async fn get_mode_handler(state: web::Data<ruxel::State>) -> Result<web::Bytes> {
    Ok(web::Bytes::from(state.mode.lock().unwrap().to_string()))
}
async fn canvas_info_handler(state: web::Data<ruxel::State>) -> Result<web::Bytes> {
    let canvas = state.canvas.lock().unwrap();

    Ok(web::Bytes::from(format!(
        "\"width\":{},\"height\":{}",
        canvas.width(),
        canvas.height()
    )))
}
