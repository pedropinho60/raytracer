use derive_more::From;

use crate::{core::color::Color, parse::dto::MaterialDTO};

#[derive(Clone, From)]
pub enum Material {
    Flat { kd: Color },
    Checkerboard(CheckerboardMaterial),
    BlinnPhong(BlinnPhongMaterial),
    Cel(CelMaterial),
}

impl From<MaterialDTO> for Material {
    fn from(value: MaterialDTO) -> Self {
        match value {
            MaterialDTO::Flat { color } => Material::Flat { kd: color.into() },
            MaterialDTO::Checkerboard {
                color_a,
                color_b,
                scale,
            } => CheckerboardMaterial::new(color_a.into(), color_b.into(), scale).into(),
            MaterialDTO::Blinn {
                ambient,
                diffuse,
                specular,
                glossiness,
                mirror,
            } => BlinnPhongMaterial::new(diffuse, specular, glossiness, ambient, mirror).into(),
            MaterialDTO::Cel {
                color_map,
                ambient,
                shadow_color,
                silhouette_angle,
                silhouette_color,
            } => CelMaterial::new(
                color_map.0.into_iter().rev().collect(),
                ambient,
                shadow_color.map(Color::from),
                silhouette_angle,
                silhouette_color,
            )
            .into(),
        }
    }
}

#[derive(Clone)]
pub struct CheckerboardMaterial {
    color_a: Color,
    color_b: Color,
    scale: f32,
}

impl CheckerboardMaterial {
    pub fn new(color_a: Color, color_b: Color, scale: f32) -> Self {
        Self {
            color_a,
            color_b,
            scale: 1.0 / scale,
        }
    }

    pub fn color_at(&self, u: f32, v: f32) -> Color {
        let u_cell = (u * self.scale).floor() as i32;
        let v_cell = (v * self.scale).floor() as i32;

        if (u_cell + v_cell) % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[derive(Clone)]
pub struct BlinnPhongMaterial {
    pub diffuse: Color,
    pub specular: Color,
    pub glossiness: u16,
    pub ambient: Color,
    pub mirror: Color,
}

impl BlinnPhongMaterial {
    pub fn new(
        diffuse: Color,
        specular: Color,
        glossiness: u16,
        ambient: Color,
        mirror: Color,
    ) -> Self {
        Self {
            diffuse,
            specular,
            glossiness,
            ambient,
            mirror,
        }
    }
}

#[derive(Clone)]
pub struct CelMaterial {
    pub color_map: Vec<Color>,
    pub ambient: Color,
    pub shadow_color: Color,
    pub silhouette_cos: Option<f32>,
    pub silhouette_color: Color,
}

impl CelMaterial {
    pub fn new(
        color_map: Vec<Color>,
        ambient: Color,
        shadow_color: Option<Color>,
        silhouette_angle: Option<f32>,
        silhouette_color: Color,
    ) -> Self {
        Self {
            color_map,
            ambient,
            shadow_color: shadow_color.unwrap_or(Color::BLACK),
            silhouette_cos: silhouette_angle.map(|a| a.to_radians().cos()),
            silhouette_color,
        }
    }
}
