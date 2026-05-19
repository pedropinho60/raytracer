use derive_more::From;

use crate::{
    core::{color::Color, ray::Ray},
    parse::dto::BackgroundDTO,
};

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

impl From<BackgroundDTO> for Background {
    fn from(value: BackgroundDTO) -> Self {
        match value {
            BackgroundDTO::SingleColor { color } => SingleColorBackground {
                color: color.into(),
            }
            .into(),
            BackgroundDTO::FourColors { bl, tl, tr, br } => {
                GradientBackground::new(bl.into(), tl.into(), tr.into(), br.into()).into()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SingleColorBackground {
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct GradientBackground {
    bl: Color,
    tl: Color,
    tr: Color,
    br: Color,
}

impl GradientBackground {
    pub fn new(bl: Color, tl: Color, tr: Color, br: Color) -> Self {
        Self { bl, tl, tr, br }
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
