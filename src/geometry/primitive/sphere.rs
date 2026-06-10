use glam::Vec3A;

use crate::{
    core::ray::Ray,
    geometry::{bounding_box::BoundingBox, surfel::HitRecord},
};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
}

impl Sphere {
    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.center - self.radius, self.center + self.radius)
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<HitRecord> {
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
        if t < ray.t_min || t > ray.t_max {
            t = t2;
            if t < ray.t_min || t > ray.t_max {
                return None;
            }
        }

        let point = ray.origin + ray.direction * t;

        let outward_vector = point - self.center;

        let normal = outward_vector / self.radius;

        let from_behind = ray.direction.dot(normal) > 0.0;

        if from_behind {
            return None;
        }

        let d = -normal;

        let u = 0.5 + d.z.atan2(d.x) * 0.5 * std::f32::consts::FRAC_1_PI;
        let v = 0.5 + d.y.asin() * std::f32::consts::FRAC_1_PI;

        Some(HitRecord {
            point,
            normal,
            u,
            v,
            t,
        })
    }

    pub fn intersect_any(&self, ray: &mut Ray) -> bool {
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
        if t1 > ray.t_min && t1 < ray.t_max {
            return true;
        }

        let t2 = t_c + t_half;
        if t2 > ray.t_min && t2 < ray.t_max {
            return true;
        }

        false
    }
}
