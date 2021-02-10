use super::unit;
#[path = "color.rs"]
mod color;
pub use color::Rgba;

#[derive(Debug, Clone, Copy)]
struct X<T: std::ops::Add> {
    r: T,
}
impl<T: std::ops::Add<Output = T>> X<T> {
    pub fn new(r: T) -> Self {
        Self { r }
    }
    pub fn add(self, b: Self) -> Self {
        Self { r: self.r + b.r }
    }
}

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

pub fn pixel(upper: Rgba, lower: Rgba) -> String {
    let (upper_r, upper_g, upper_b, _) = upper.rgba();
    let (lower_r, lower_g, lower_b, _) = lower.rgba();
    format!(
        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m\u{2580}\x1b[0m",
        upper_r, upper_g, upper_b, lower_r, lower_g, lower_b,
    )
}
pub fn pixel_bottom(upper: Rgba) -> String {
    let (upper_r, upper_g, upper_b, _) = upper.rgba();
    format!(
        "\x1B[38;2;{};{};{}m\u{2580}\x1B[0m",
        upper_r, upper_g, upper_b
    )
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
