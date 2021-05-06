use image::{self};
use std::sync::Mutex;

pub struct State {
    pub canvas: Mutex<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pub mode: Mutex<String>,
}
