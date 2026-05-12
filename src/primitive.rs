use derive_more::From;
use glam::Vec3A;

use crate::{bounding_box::BoundingBox, hittable::Hit, ray::Ray, surfel::Surfel};

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
            Shape::Plane(inner) => inner.bounding_box(),
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

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
}

impl Sphere {
    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.center - self.radius, self.center + self.radius)
    }

    pub fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3A, Vec3A, bool)> {
        let o = ray.origin;

        let oc = o - self.center;

        let parallel_len = oc.dot(ray.direction);
        let oc_perp = oc - ray.direction * parallel_len;

        let delta = self.radius * self.radius - oc_perp.dot(oc_perp);

        if delta < 0.0 {
            return None;
        }

        let t_c = -parallel_len;

        let t_half = delta.sqrt();

        let t1 = t_c - t_half;
        let t2 = t_c + t_half;

        let mut t = t1;
        if t < t_min || t > t_max {
            t = t2;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.origin + ray.direction * t;

        let outward_vector = point - self.center;

        let normal = outward_vector / self.radius;
        let from_behind = ray.direction.dot(normal) > 0.0;

        Some((t, point, normal, from_behind))
    }

    pub fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        let oc = ray.origin - self.center;

        let parallel_len = oc.dot(ray.direction);
        let oc_perp = oc - ray.direction * parallel_len;

        let delta = self.radius * self.radius - oc_perp.dot(oc_perp);

        if delta < 0.0 {
            return false;
        }

        let t_c = -parallel_len;
        let t_half = delta.sqrt();

        let t1 = t_c - t_half;
        if t1 > t_min && t1 < t_max {
            return true;
        }

        let t2 = t_c + t_half;
        if t2 > t_min && t2 < t_max {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    point: Vec3A,
    normal: Vec3A,
}

impl Plane {
    pub fn new(point: Vec3A, normal: Vec3A) -> Self {
        Self {
            point,
            normal: normal.normalize(),
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::UNIVERSE
    }

    pub fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3A, Vec3A, bool)> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < 1e-6 {
            return None;
        }

        let p0l0 = self.point - ray.origin;
        let t = p0l0.dot(self.normal) / denom;

        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.origin + ray.direction * t;

        let normal = self.normal;
        let from_behind = ray.direction.dot(normal) > 0.0;

        Some((t, point, normal, from_behind))
    }

    pub fn intersect_any(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < 1e-6 {
            return false;
        }

        let t = (self.point - ray.origin).dot(self.normal) / denom;

        t > t_min && t < t_max
    }
}
