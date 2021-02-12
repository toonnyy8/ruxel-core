#[path = "tui.rs"]
pub mod tui;

#[path = "command.rs"]
pub mod command;

#[path ="unit.rs"]
pub mod unit;

#[path ="file.rs"]
pub mod file;

use im_rc as im;

pub struct Canvas_ {
    data: im::Vector<tui::Rgba>,
    size: unit::Position,
}

impl Canvas_ {
    pub fn new(size: unit::Position) -> Self {
        let mut data = im::Vector::<tui::Rgba>::new();
        for _ in 0..(size.x * size.y) {
            data.push_back(tui::Rgba::default());
        }
        Self { data, size }
    }
    pub fn update(&self, (x, y): (usize, usize), pixel: tui::Rgba) -> Self {
        Self {
            data: self.data.update(y * self.size.x as usize + x, pixel),
            size: self.size,
        }
    }
}
