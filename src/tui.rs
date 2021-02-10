use super::unit;
#[path = "color.rs"]
mod color;
pub use color::Rgba;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub data: Vec<Vec<Rgba>>,
    pub size: unit::Position,
}

impl Canvas {
    pub fn new(size: unit::Position) -> Self {
        let mut data = Vec::new();
        for _ in 0..size.y {
            let mut row = Vec::new();
            row.resize(size.x as usize, Rgba::default());
            data.push(row);
        }
        Self { data, size }
    }
}

pub fn pixel_fg(rgba: Rgba, s: &str) -> String {
    let (r, g, b, _) = rgba.into();
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, s)
}
pub fn pixel_bg(rgba: Rgba, s: &str) -> String {
    let (r, g, b, _) = rgba.into();
    format!("\x1B[48;2;{};{};{}m{}\x1B[0m", r, g, b, s)
}
pub fn pixel_both(fg_rgb: Rgba, bg_rgb: Rgba, s: &str) -> String {
    pixel_fg(fg_rgb, &pixel_bg(bg_rgb, s))
}

pub fn clear_up(line_num: i64) {
    if line_num != 0 {
        print!("\x1B[{}F", line_num);
    }
    for _ in 0..line_num {
        print!("\x1B[2K\n");
    }
    if line_num != 0 {
        print!("\x1B[{}F", line_num);
    }
}

pub fn clear_down(line_num: i64) {
    for _ in 0..line_num {
        print!("\x1B[2K\n");
    }
    if line_num != 0 {
        print!("\x1B[{}F", line_num);
    }
}
