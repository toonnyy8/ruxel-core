#[derive(Debug, Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}
impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
    fn to_interim(&self) -> InterimRGB {
        InterimRGB::new(self.r as u16, self.g as u16, self.b as u16)
    }
}
impl std::cmp::PartialEq for Rgb {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}
fn u16_to_u8(item: u16) -> u8 {
    if item > 255 {
        255
    } else {
        item as u8
    }
}

#[derive(Debug, Clone, Copy)]
struct InterimRGB {
    r: u16,
    g: u16,
    b: u16,
}
impl InterimRGB {
    fn new(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }
    fn to_rgb(&self) -> Rgb {
        Rgb::new(u16_to_u8(self.r), u16_to_u8(self.g), u16_to_u8(self.b))
    }
    fn lightness(&self) -> u16 {
        (self.r + self.g + self.b) / 3
    }
}
impl std::ops::Add<InterimRGB> for InterimRGB {
    type Output = InterimRGB;

    fn add(self, _rhs: InterimRGB) -> InterimRGB {
        let r = self.r + _rhs.r;
        let g = self.g + _rhs.g;
        let b = self.b + _rhs.b;
        InterimRGB::new(r, g, b)
    }
}
impl std::ops::Mul<u16> for InterimRGB {
    type Output = InterimRGB;

    fn mul(self, _rhs: u16) -> InterimRGB {
        let r = self.r * _rhs;
        let g = self.g * _rhs;
        let b = self.b * _rhs;
        InterimRGB::new(r, g, b)
    }
}
impl std::ops::Div<u16> for InterimRGB {
    type Output = InterimRGB;

    fn div(self, _rhs: u16) -> InterimRGB {
        let r = self.r / _rhs;
        let g = self.g / _rhs;
        let b = self.b / _rhs;
        InterimRGB::new(r, g, b)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    rgb: Rgb,
    alpha: u8,
}
impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self {
            rgb: Rgb::new(r, g, b),
            alpha,
        }
    }
    pub fn default() -> Self {
        Self {
            rgb: Rgb::default(),
            alpha: 255,
        }
    }

    pub fn compositing(&self, bg: &Rgba) -> Self {
        let fg = self;

        let fg_alpha = fg.alpha as u16;
        let bg_alpha = bg.alpha as u16;

        let fg_rgb = fg.rgb.to_interim();
        let bg_rgb = bg.rgb.to_interim();

        let _alpha = bg_alpha * (255 - fg_alpha) / 255;
        let alpha = fg_alpha + _alpha;

        let rgb = if alpha == 0 {
            Rgb::new(0, 0, 0)
        } else {
            let interim_rgb: InterimRGB = (fg_rgb * fg_alpha + bg_rgb * _alpha) / alpha;
            interim_rgb.to_rgb()
        };

        Self {
            rgb: rgb,
            alpha: u16_to_u8(alpha),
        }
    }
    pub fn lightness(&self) -> u8 {
        u16_to_u8(self.rgb.to_interim().lightness() * self.alpha as u16 / 255)
    }
}
impl std::cmp::PartialEq for Rgba {
    fn eq(&self, other: &Self) -> bool {
        self.rgb == other.rgb && self.alpha == other.alpha
    }
}
impl From<Rgba> for (u8, u8, u8, u8) {
    fn from(item: Rgba) -> (u8, u8, u8, u8) {
        (item.rgb.r, item.rgb.g, item.rgb.b, item.alpha)
    }
}
impl From<Rgba> for [u8; 4] {
    fn from(item: Rgba) -> [u8; 4] {
        [item.rgb.r, item.rgb.g, item.rgb.b, item.alpha]
    }
}
