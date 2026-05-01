use derive_more::From;

use crate::color::Color;

#[derive(Debug, Clone, From)]
pub enum Background {
    SingleColor(SingleColorBackground),
    Gradient(GradientBackground),
}

impl Background {
    pub fn sample(&self, u: f32, v: f32) -> Color {
        match self {
            Background::SingleColor(inner) => inner.sample(),
            Background::Gradient(inner) => inner.sample(u, v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SingleColorBackground {
    color: Color,
}

impl SingleColorBackground {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn sample(&self) -> Color {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct GradientBackground {
    tl: Color,
    tr: Color,
    bl: Color,
    br: Color,
}

impl GradientBackground {
    pub fn new(tl: Color, tr: Color, bl: Color, br: Color) -> Self {
        Self { tl, tr, bl, br }
    }

    fn lerp(a: Color, b: Color, t: f32) -> Color {
        Color {
            red: (1. - t) * a.red + t * b.red,
            green: (1. - t) * a.green + t * b.green,
            blue: (1. - t) * a.blue + t * b.blue,
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let top = Self::lerp(self.tl, self.tr, v);
        let bot = Self::lerp(self.bl, self.br, v);

        Self::lerp(top, bot, u)
    }
}
