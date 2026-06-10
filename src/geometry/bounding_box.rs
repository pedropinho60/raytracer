use glam::Vec3A;

use crate::core::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl BoundingBox {
    pub const EMPTY: Self = Self {
        min: Vec3A::INFINITY,
        max: Vec3A::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: Vec3A::NEG_INFINITY,
        max: Vec3A::INFINITY,
    };

    pub fn new(a: Vec3A, b: Vec3A) -> Self {
        let mut bbox = Self {
            min: a.min(b),
            max: b.max(a),
        };
        bbox.pad_to_minimums();
        bbox
    }

    pub fn join(a: BoundingBox, b: BoundingBox) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn expand_by_box(&mut self, other: &BoundingBox) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }

    pub fn expand_by_point(&mut self, point: Vec3A) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    fn pad_to_minimums(&mut self) {
        let delta = 1e-4;

        if self.max.x - self.min.x < delta {
            self.min.x -= delta;
            self.max.x += delta;
        }
        if self.max.y - self.min.y < delta {
            self.min.y -= delta;
            self.max.y += delta;
        }
        if self.max.z - self.min.z < delta {
            self.min.z -= delta;
            self.max.z += delta;
        }
    }

    pub fn longest_axis(&self) -> u8 {
        if self.max.x - self.min.x > self.max.y - self.min.y {
            if self.max.x - self.min.x > self.max.z - self.min.z {
                0
            } else {
                2
            }
        } else {
            if self.max.y - self.min.y > self.max.z - self.min.z {
                1
            } else {
                2
            }
        }
    }

    pub fn longest_axis_len(&self) -> f32 {
        let axis = self.longest_axis() as usize;

        self.max[axis] - self.min[axis]
    }

    pub fn hit(&self, ray: &mut Ray) -> bool {
        let mut t_min = ray.t_min;
        let mut t_max = ray.t_max;

        for axis in 0..3 {
            let inv_d = ray.inv_dir[axis];

            let t0 = (self.min[axis] - ray.origin[axis]) * inv_d;
            let t1 = (self.max[axis] - ray.origin[axis]) * inv_d;

            let (t_near, t_far) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            t_min = if t_near > t_min { t_near } else { t_min };
            t_max = if t_far < t_max { t_far } else { t_max };

            if t_max < t_min {
                return false;
            }
        }

        true
    }

    #[inline]
    pub fn surface_area(&self) -> f32 {
        let d = self.max - self.min;

        if d.x < 0.0 || d.y < 0.0 || d.z < 0.0 {
            return 0.0;
        }

        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }
}
