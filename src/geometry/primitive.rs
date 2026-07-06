mod plane;
mod sphere;
mod triangle;

use std::sync::Arc;

use derive_more::From;

use glam::Vec3A;
pub use plane::Plane;
pub use sphere::Sphere;
pub use triangle::Triangle;
pub use triangle::TriangleMesh;

use crate::api::Transform;
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
    transform: Arc<Transform>,
}

impl Primitive {
    pub fn new(shape: Shape, material_id: usize, transform: Arc<Transform>) -> Self {
        Self {
            shape,
            material_id,
            transform,
        }
    }
}

impl Hit for Primitive {
    fn bounding_box(&self) -> BoundingBox {
        let local_box = self.shape.bounding_box();

        let min = local_box.min;
        let max = local_box.max;

        let corners = [
            Vec3A::new(min.x, min.y, min.z),
            Vec3A::new(max.x, min.y, min.z),
            Vec3A::new(min.x, max.y, min.z),
            Vec3A::new(max.x, max.y, min.z),
            Vec3A::new(min.x, min.y, max.z),
            Vec3A::new(max.x, min.y, max.z),
            Vec3A::new(min.x, max.y, max.z),
            Vec3A::new(max.x, max.y, max.z),
        ];

        let mut world_min = Vec3A::INFINITY;
        let mut world_max = Vec3A::NEG_INFINITY;

        for &corner in &corners {
            let transformed_corner = self.transform.obj_to_world.transform_point3a(corner);
            world_min = world_min.min(transformed_corner);
            world_max = world_max.max(transformed_corner);
        }

        BoundingBox {
            min: world_min,
            max: world_max,
        }
    }

    fn intersect(&self, ray: &mut Ray) -> Option<Surfel> {
        let o = self.transform.world_to_obj.transform_point3a(ray.origin);
        let d = self
            .transform
            .world_to_obj
            .transform_vector3a(ray.direction);

        let mut obj_ray = Ray {
            origin: o,
            direction: d,
            ..*ray
        };

        let hit = self.shape.intersect(&mut obj_ray)?;

        ray.t_max = obj_ray.t_max;

        let world_point = self.transform.obj_to_world.transform_point3a(hit.point);

        let inverse_linear_transform = self.transform.world_to_obj.matrix3;
        let world_normal = (inverse_linear_transform.transpose() * hit.normal).normalize();

        Some(Surfel {
            point: world_point,
            normal: world_normal,
            u: hit.u,
            v: hit.v,
            t: hit.t,
            material_id: self.material_id,
        })
    }

    fn intersect_any(&self, ray: &mut Ray) -> bool {
        let obj_origin = self.transform.world_to_obj.transform_point3a(ray.origin);
        let obj_direction = self
            .transform
            .world_to_obj
            .transform_vector3a(ray.direction);

        let mut obj_ray = Ray {
            origin: obj_origin,
            direction: obj_direction,
            ..*ray
        };

        self.shape.intersect_any(&mut obj_ray)
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
