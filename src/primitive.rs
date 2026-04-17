use derive_more::From;

use crate::{
    math::{Point3, Vec3},
    ray::Ray,
    surfel::Surfel,
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

    pub fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<(f64, Surfel)> {
        let (t, p, n) = self.shape.intersect(ray, t_min, t_max)?;

        Some((
            t,
            Surfel {
                p,
                n,
                material_id: self.material_id,
            },
        ))
    }
}

#[derive(Debug, Clone, From)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<(f64, Point3, Vec3)> {
        match self {
            Shape::Sphere(inner) => inner.intersect(ray, t_min, t_max),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn intersect(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<(f64, Point3, Vec3)> {
        let o = ray.origin;
        let d_hat = ray.direction.normalize();

        let oc = o - self.center;

        let parallel_len = oc.dot(d_hat);
        let oc_perp = oc - d_hat * parallel_len;

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

        let point = ray.origin + d_hat * t;

        let outward_vector = point - self.center;

        let mut normal = outward_vector / self.radius;

        if ray.direction.dot(normal) > 0.0 {
            normal = -normal;
        }

        Some((t, point, normal))
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
        let mut t_closest = f64::INFINITY;

        for primitive in &self.primitives {
            if let Some((t, surfel)) = primitive.intersect(ray, t_min, t_closest) {
                t_closest = t;
                closest_hit = Some(surfel);
            }
        }

        closest_hit
    }
}
