mod plane;
mod sphere;
mod triangle;

use derive_more::From;

pub use plane::Plane;
pub use sphere::Sphere;
pub use triangle::Triangle;
pub use triangle::TriangleMesh;

use crate::{
    core::ray::Ray,
    geometry::{
        bounding_box::BoundingBox,
        hittable::Hit,
        surfel::{HitRecord, Surfel},
    },
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

    fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Surfel> {
        let hit = self.shape.intersect(ray, t_min, t_max)?;

        Some(Surfel {
            point: hit.point,
            normal: hit.normal,
            u: hit.u,
            v: hit.v,
            t: hit.t,
            material_id: self.material_id,
        })
    }

    fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        self.shape.intersect_any(ray, t_min, t_max)
    }
}

#[derive(Debug, Clone, From)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
}

impl Shape {
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            Shape::Sphere(inner) => inner.bounding_box(),
            Shape::Plane(_) => BoundingBox::UNIVERSE,
            Shape::Triangle(inner) => inner.bounding_box(),
        }
    }
    pub fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Shape::Sphere(inner) => inner.intersect(ray, t_min, t_max),
            Shape::Plane(inner) => inner.intersect(ray, t_min, t_max),
            Shape::Triangle(inner) => inner.intersect(ray, t_min, t_max),
        }
    }

    pub fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        match self {
            Shape::Sphere(inner) => inner.intersect_any(ray, t_min, t_max),
            Shape::Plane(inner) => inner.intersect_any(ray, t_min, t_max),
            Shape::Triangle(inner) => inner.intersect_any(ray, t_min, t_max),
        }
    }
}
