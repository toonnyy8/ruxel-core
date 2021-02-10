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

// #[derive(Debug, Clone, Copy)]
// pub struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
//     a: u8,
// }
// impl Color {
//     pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
//         Self { r, g, b, a }
//     }
//     pub fn default() -> Self {
//         Self {
//             r: 0,
//             g: 0,
//             b: 0,
//             a: 0,
//         }
//     }
//     pub fn mix(a: Color, b: Color) -> Self {
//         Self {
//             r: a.r / 2 + b.r / 2,
//             g: a.g / 2 + b.g / 2,
//             b: a.b / 2 + b.b / 2,
//             a: a.a / 2 + b.a / 2,
//         }
//     }

//     pub fn lightness(a: Color) -> u8 {
//         return ((a.r as u16 + a.g as u16 + a.b as u16) / 3) as u8;
//     }
// }
// impl std::cmp::PartialEq for Color {
//     fn eq(&self, other: &Self) -> bool {
//         self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
//     }
// }
// impl std::ops::Add<Color> for Color {
//     type Output = Color;

//     fn add(self, _rhs: Color) -> Color {
//         Self {
//             r: self.r + _rhs.r,
//             g: self.g + _rhs.g,
//             b: self.b + _rhs.b,
//             a: self.a + _rhs.a,
//         }
//     }
// }
// impl std::ops::Sub<Color> for Color {
//     type Output = Color;

//     fn sub(self, _rhs: Color) -> Color {
//         Self {
//             r: self.r - _rhs.r,
//             g: self.g - _rhs.g,
//             b: self.b - _rhs.b,
//             a: self.a - _rhs.a,
//         }
//     }
// }
// impl std::ops::Div<f64> for Color {
//     type Output = Color;

//     fn div(self, _rhs: f64) -> Color {
//         Self {
//             r: (self.r as f64 / _rhs) as u8,
//             g: (self.g as f64 / _rhs) as u8,
//             b: (self.b as f64 / _rhs) as u8,
//             a: (self.a as f64 / _rhs) as u8,
//         }
//     }
// }

// impl std::ops::Mul<f64> for Color {
//     type Output = Color;

//     fn mul(self, _rhs: f64) -> Color {
//         Self {
//             r: (self.r as f64 * _rhs) as u8,
//             g: (self.g as f64 * _rhs) as u8,
//             b: (self.b as f64 * _rhs) as u8,
//             a: (self.a as f64 * _rhs) as u8,
//         }
//     }
// }

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
