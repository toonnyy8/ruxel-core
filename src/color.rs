fn clip(value: f32, max: f32, min: f32) -> f32 {
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

#[derive(Debug, Clone, Copy)]
struct ColorRGBf32 {
    r: f32,
    g: f32,
    b: f32,
}
impl ColorRGBf32 {
    fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: clip(r, 1., 0.),
            g: clip(g, 1., 0.),
            b: clip(b, 1., 0.),
        }
    }
    fn default() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }
    fn to_u8(&self) -> ColorRGB {
        ColorRGB::new(
            (self.r * 255.) as u8,
            (self.g * 255.) as u8,
            (self.b * 255.) as u8,
        )
    }
    fn lightness(&self) -> f32 {
        (self.r + self.g + self.b) / 3.
    }
}
impl std::ops::Add<ColorRGBf32> for ColorRGBf32 {
    type Output = ColorRGBf32;

    fn add(self, _rhs: ColorRGBf32) -> ColorRGBf32 {
        let r = self.r + _rhs.r;
        let g = self.g + _rhs.g;
        let b = self.b + _rhs.b;
        ColorRGBf32::new(r, g, b)
    }
}
impl std::ops::Mul<f32> for ColorRGBf32 {
    type Output = ColorRGBf32;

    fn mul(self, _rhs: f32) -> ColorRGBf32 {
        let r = self.r * _rhs;
        let g = self.g * _rhs;
        let b = self.b * _rhs;
        ColorRGBf32::new(r, g, b)
    }
}
impl std::ops::Div<f32> for ColorRGBf32 {
    type Output = ColorRGBf32;

    fn div(self, _rhs: f32) -> ColorRGBf32 {
        let r = self.r / _rhs;
        let g = self.g / _rhs;
        let b = self.b / _rhs;
        ColorRGBf32::new(r, g, b)
    }
}

#[derive(Debug, Clone, Copy)]
struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}
impl ColorRGB {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
    fn to_f32(&self) -> ColorRGBf32 {
        ColorRGBf32::new(
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
        )
    }

    fn lightness(&self) -> u8 {
        (self.to_f32().lightness() * 255.) as u8
    }
}
impl std::cmp::PartialEq for ColorRGB {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorRGBA {
    rgb: ColorRGB,
    alpha: u8,
}
impl ColorRGBA {
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self {
            rgb: ColorRGB::new(r, g, b),
            alpha,
        }
    }
    pub fn default() -> Self {
        Self {
            rgb: ColorRGB::default(),
            alpha: 255,
        }
    }

    pub fn compositing(&self, bg: &ColorRGBA) -> Self {
        let fg = self;

        let fg_alpha = fg.alpha as f32 / 255.;
        let bg_alpha = bg.alpha as f32 / 255.;

        let fg_rgb = fg.rgb.to_f32();
        let bg_rgb = bg.rgb.to_f32();

        let _alpha = bg_alpha * (1. - fg_alpha);
        let alpha = fg_alpha + _alpha;

        let rgb = if alpha == 0. {
            ColorRGBf32::default()
        } else {
            (fg_rgb * fg_alpha + bg_rgb * _alpha) / alpha
        };

        Self {
            rgb: rgb.to_u8(),
            alpha: (alpha * 255.) as u8,
        }
    }
    pub fn show(&self) {
        println!("r:{}", self.rgb.r);
        println!("g:{}", self.rgb.g);
        println!("b:{}", self.rgb.b);
        println!("a:{}", self.alpha);
    }
    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.rgb.r, self.rgb.g, self.rgb.b, self.alpha)
    }
    pub fn lightness(&self) -> u8 {
        (self.rgb.lightness() as f32 * self.alpha as f32 / 255.) as u8
    }
}
impl std::cmp::PartialEq for ColorRGBA {
    fn eq(&self, other: &Self) -> bool {
        self.rgb == other.rgb && self.alpha == other.alpha
    }
}
