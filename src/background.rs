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

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let top = Color::lerp(self.tl, self.tr, v);
        let bot = Color::lerp(self.bl, self.br, v);

        Color::lerp(top, bot, u)
    }
}
