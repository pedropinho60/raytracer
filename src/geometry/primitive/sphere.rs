use glam::Vec3A;

use crate::{core::ray::Ray, geometry::bounding_box::BoundingBox};

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
