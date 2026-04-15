use crate::{math::Point3, ray::Ray};

pub trait Object {
    fn intersect_p(&self, ray: Ray) -> bool;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Object for Sphere {
    fn intersect_p(&self, ray: Ray) -> bool {
        let o = ray.origin;
        let d_hat = ray.direction.normalize();

        let oc = o - self.center;

        let parallel_len = oc.dot(d_hat);
        let oc_perp = oc - d_hat * parallel_len;

        let delta = self.radius * self.radius - oc_perp.dot(oc_perp);

        delta >= 0.0
    }
}
