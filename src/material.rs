use derive_more::From;

use crate::{RGBColor, math::Point3};

#[derive(Clone, From)]
pub enum Material {
    Flat { kd: RGBColor },
    Checkerboard(CheckerboardMaterial),
}

impl Material {
    pub fn color_at(&self, point: Point3) -> RGBColor {
        match self {
            Material::Flat { kd } => *kd,
            Material::Checkerboard(checkerboard_material) => checkerboard_material.color_at(point),
        }
    }
}

#[derive(Clone)]
pub struct CheckerboardMaterial {
    color_a: RGBColor,
    color_b: RGBColor,
    scale: f64,
}

impl CheckerboardMaterial {
    pub fn new(color_a: RGBColor, color_b: RGBColor, scale: f64) -> Self {
        Self {
            color_a,
            color_b,
            scale,
        }
    }

    pub fn color_at(&self, point: Point3) -> RGBColor {
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
