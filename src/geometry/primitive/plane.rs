use glam::Vec3A;

use crate::{core::ray::Ray, geometry::surfel::HitRecord};

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

    pub fn intersect(&self, ray: &mut Ray) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < 1e-6 {
            return None;
        }

        let p0l0 = self.point - ray.origin;
        let t = p0l0.dot(self.normal) / denom;

        if t < ray.t_min || t > ray.t_max {
            return None;
        }

        let point = ray.origin + ray.direction * t;

        let normal = self.normal;
        let from_behind = ray.direction.dot(normal) > 0.0;

        if from_behind {
            return None;
        }

        let tangent = if normal.x.abs() > 0.9 {
            Vec3A::Y
        } else {
            Vec3A::X
        };

        let u_axis = normal.cross(tangent).normalize();
        let v_axis = normal.cross(u_axis);

        let hit_vec = point - self.point;

        let u = hit_vec.dot(u_axis);
        let v = hit_vec.dot(v_axis);

        Some(HitRecord {
            point,
            normal,
            u,
            v,
            t,
        })
    }

    pub fn intersect_any(&self, ray: &mut Ray) -> bool {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < 1e-6 {
            return false;
        }

        let t = (self.point - ray.origin).dot(self.normal) / denom;

        t > ray.t_min && t < ray.t_max
    }
}
