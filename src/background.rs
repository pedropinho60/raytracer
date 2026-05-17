use derive_more::From;

use crate::{color::Color, ray::Ray};

#[derive(Debug, Clone, From)]
pub enum Background {
    SingleColor(SingleColorBackground),
    Gradient(GradientBackground),
}

impl Background {
    pub fn sample(&self, u: f32, v: f32) -> Color {
        match self {
            Background::SingleColor(inner) => inner.color,
            Background::Gradient(inner) => inner.sample(u, v),
        }
    }

    pub fn sample_ray(&self, ray: Ray) -> Color {
        match self {
            Background::SingleColor(inner) => inner.color,
            Background::Gradient(inner) => inner.sample_ray(ray),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SingleColorBackground {
    pub color: Color,
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

    pub fn sample_ray(&self, ray: Ray) -> Color {
        let d = ray.direction.normalize();

        let u = 0.5 - d.y * 0.5;
        let v = 0.5;

        self.sample(u, v)
    }
}
