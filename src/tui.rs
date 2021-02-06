#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
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
