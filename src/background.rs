use crate::RGBColor;

pub trait Background {
    fn sample(&self, u: f64, v: f64) -> RGBColor;
}

pub struct SingleColorBackground {
    color: RGBColor,
}

impl SingleColorBackground {
    pub fn new(color: RGBColor) -> Self {
        Self { color }
    }
}

impl Background for SingleColorBackground {
    fn sample(&self, _u: f64, _v: f64) -> RGBColor {
        self.color
    }
}

pub struct GradientBackground {
    tl: RGBColor,
    tr: RGBColor,
    bl: RGBColor,
    br: RGBColor,
}

impl GradientBackground {
    pub fn new(tl: RGBColor, tr: RGBColor, bl: RGBColor, br: RGBColor) -> Self {
        Self { tl, tr, bl, br }
    }

    fn lerp(a: RGBColor, b: RGBColor, t: f64) -> RGBColor {
        RGBColor {
            red: ((1. - t) * a.red as f64 + t * b.red as f64) as u8,
            green: ((1. - t) * a.green as f64 + t * b.green as f64) as u8,
            blue: ((1. - t) * a.blue as f64 + t * b.blue as f64) as u8,
        }
    }
}

impl Background for GradientBackground {
    fn sample(&self, u: f64, v: f64) -> RGBColor {
        let top = Self::lerp(self.tl, self.tr, v);
        let bot = Self::lerp(self.bl, self.br, v);

        Self::lerp(top, bot, u)
    }
}
