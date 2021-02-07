use super::unit;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
    pub fn mix(a: Color, b: Color) -> Self {
        Self {
            r: a.r / 2 + b.r / 2,
            g: a.g / 2 + b.g / 2,
            b: a.b / 2 + b.b / 2,
            a: a.a / 2 + b.a / 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    pub data: Vec<Vec<Color>>,
    pub size: unit::Position,
}

impl Canvas {
    pub fn new(size: unit::Position) -> Self {
        let mut data = Vec::new();
        for _ in 0..size.y {
            let mut row = Vec::new();
            row.resize(size.x as usize, Color::default());
            data.push(row);
        }
        Self { data, size }
    }
}

pub fn pixel(upper: Color, lower: Color) -> String {
    format!(
        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m\u{2580}\x1b[0m",
        upper.r, upper.g, upper.b, lower.r, lower.g, lower.b
    )
}
pub fn pixel_bottom(upper: Color) -> String {
    format!(
        "\x1B[38;2;{};{};{}m\u{2580}\x1B[0m",
        upper.r, upper.g, upper.b
    )
}
