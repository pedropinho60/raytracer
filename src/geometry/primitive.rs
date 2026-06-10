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

    fn intersect(&self, ray: &mut Ray) -> Option<Surfel> {
        let hit = self.shape.intersect(ray)?;

        Some(Surfel {
            point: hit.point,
            normal: hit.normal,
            u: hit.u,
            v: hit.v,
            t: hit.t,
            material_id: self.material_id,
        })
    }

    fn intersect_any(&self, ray: &mut Ray) -> bool {
        self.shape.intersect_any(ray)
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
    pub fn intersect(&self, ray: &mut Ray) -> Option<HitRecord> {
        match self {
            Shape::Sphere(inner) => inner.intersect(ray),
            Shape::Plane(inner) => inner.intersect(ray),
            Shape::Triangle(inner) => inner.intersect(ray),
        }
    }

    pub fn intersect_any(&self, ray: &mut Ray) -> bool {
        match self {
            Shape::Sphere(inner) => inner.intersect_any(ray),
            Shape::Plane(inner) => inner.intersect_any(ray),
            Shape::Triangle(inner) => inner.intersect_any(ray),
        }
    }
}
