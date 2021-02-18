use super::im;
use super::position::Position;
use super::Rgba;
#[derive(Debug, Clone)]
pub struct Canvas {
    data: im::Vector<Rgba>,
    w: u32,
    h: u32,
}

impl Canvas {
    pub fn new((w, h): (u32, u32)) -> Self {
        let mut data = im::Vector::new();
        for _ in 0..(w * h) {
            data.push_back(Rgba::default())
        }
        Self { data, w, h }
    }
    pub fn update(&self, pos: &Position, color: Rgba) -> Self {
        let (w, h) = self.size();
        let (x, y) = pos.into();
        Self {
            data: self.data.update((y * w + x) as usize, color),
            w,
            h,
        }
    }
    pub fn size(&self) -> (u32, u32) {
        (self.w, self.h)
    }
    pub fn include(&self, pos: &Position) -> bool {
        let (w, h) = self.size();
        let (x, y) = pos.into();
        x >= 0 && x < w && y >= 0 && y < h
    }
    pub fn at(&self, pos: &Position) -> &Rgba {
        let (w, _) = self.size();
        let (x, y) = pos.into();
        self.data.get((y * w + x) as usize).unwrap()
    }
}
