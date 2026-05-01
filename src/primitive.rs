use derive_more::From;
use glam::Vec3;

use crate::{ray::Ray, surfel::Surfel};

#[derive(Debug, Clone)]
pub struct Primitive {
    shape: Shape,
    material_id: usize,
}

impl Primitive {
    pub fn new(shape: Shape, material_id: usize) -> Self {
        Self { shape, material_id }
    }

    pub fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Surfel)> {
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
}

#[derive(Debug, Clone, From)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3, Vec3, bool)> {
        match self {
            Shape::Sphere(inner) => inner.intersect(ray, t_min, t_max),
            Shape::Plane(inner) => inner.intersect(ray, t_min, t_max),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3, Vec3, bool)> {
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
}

#[derive(Debug, Clone)]
pub struct Plane {
    point: Vec3,
    normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self {
            point,
            normal: normal.normalize(),
        }
    }

    pub fn intersect(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3, Vec3, bool)> {
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
}

#[derive(Debug, Clone)]
pub struct AggregatePrimitive {
    primitives: Vec<Primitive>,
}

impl AggregatePrimitive {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
        }
    }

    pub fn add(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    pub fn intersect(&self, ray: Ray) -> Option<Surfel> {
        let mut closest_hit = None;

        let t_min = 0.001;
        let mut t_closest = f32::INFINITY;

        for primitive in &self.primitives {
            if let Some((t, surfel)) = primitive.intersect(ray, t_min, t_closest) {
                t_closest = t;
                closest_hit = Some(surfel);
            }
        }

        closest_hit
    }
}
