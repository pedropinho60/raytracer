use derive_more::From;
use glam::Vec3;

use crate::color::Color;

#[derive(Clone, From)]
pub enum Material {
    Flat { kd: Color },
    Checkerboard(CheckerboardMaterial),
    BlinnPhong(BlinnPhongMaterial),
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
            scale,
        }
    }

    pub fn color_at(&self, point: Vec3) -> Color {
        let scaled_point = point / self.scale;

        let ix = scaled_point.x.floor() as i64;
        let iy = scaled_point.y.floor() as i64;
        let iz = scaled_point.z.floor() as i64;

        if (ix + iy + iz) % 2 == 0 {
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
    pub glossiness: f32,
    pub ambient: Color,
}

impl BlinnPhongMaterial {
    pub fn new(diffuse: Color, specular: Color, glossiness: f32, ambient: Color) -> Self {
        Self {
            diffuse,
            specular,
            glossiness,
            ambient,
        }
    }
}
