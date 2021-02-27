#![feature(impl_trait_in_bindings)]
use bytes;
use reqwest;
mod ruxel;
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::thread;
use warp::Filter;

#[tokio::main]
async fn main() {
    let config = ruxel::Config::import("ruxel.json");
    config.start();
    let core_port = config.core_port();

    thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        for line in std::io::stdin().lock().lines() {
            let cmd = line.unwrap();
            if cmd == "exit" {
                exit(0);
            } else {
                config.run(&client, cmd.as_str());
            }
        }
    });

    let print_post = warp::post()
        .and(warp::path("print"))
        .and(warp::body::bytes())
        .map(|bytes: bytes::Bytes| {
            print!("{}", std::str::from_utf8(&bytes[..]).unwrap());
            io::stdout().flush().unwrap();
            warp::reply()
        });

    // let cursor_post = warp::post()
    //     .and(warp::path("get_one"))
    //     .and(warp::body::bytes())
    //     .map(|bytes: bytes::Bytes| {
    //         print!("{}", std::str::from_utf8(&bytes[..]).unwrap());
    //         io::stdout().flush().unwrap();
    //         // warp::reply()
    //         ""
    //     });

    warp::serve(print_post)
        .run(([127, 0, 0, 1], core_port))
        .await;
}
