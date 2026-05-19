mod plane;
mod sphere;

use derive_more::From;
use glam::Vec3A;

pub use plane::Plane;
pub use sphere::Sphere;

use crate::{
    core::ray::Ray,
    geometry::{bounding_box::BoundingBox, hittable::Hit, surfel::Surfel},
};

#[derive(Debug, Clone)]
pub struct Primitive {
    shape: Shape,
    material_id: usize,
}

impl Primitive {
    pub fn new(shape: Shape, material_id: usize) -> Self {
        Self { shape, material_id }
    }
}

impl Hit for Primitive {
    fn bounding_box(&self) -> BoundingBox {
        self.shape.bounding_box()
    }

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Surfel)> {
        let (t, point, normal, from_behind) = self.shape.intersect(ray, t_min, t_max)?;

        Some((
            t,
            Surfel {
                point,
                normal,
                from_behind,
                material_id: self.material_id,
            },
        ))
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        self.shape.intersect_any(ray, t_min, t_max)
    }
}

#[derive(Debug, Clone, From)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            Shape::Sphere(inner) => inner.bounding_box(),
            Shape::Plane(_) => BoundingBox::UNIVERSE,
        }
    }
    pub fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3A, Vec3A, bool)> {
        match self {
            Shape::Sphere(inner) => inner.intersect(ray, t_min, t_max),
            Shape::Plane(inner) => inner.intersect(ray, t_min, t_max),
        }
    }

    pub fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        match self {
            Shape::Sphere(inner) => inner.intersect_any(ray, t_min, t_max),
            Shape::Plane(inner) => inner.intersect_any(ray, t_min, t_max),
        }
    }
}
