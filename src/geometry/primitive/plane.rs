use glam::Vec3A;

use crate::core::ray::Ray;

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
