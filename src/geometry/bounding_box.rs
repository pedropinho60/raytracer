use std::ops::Index;

use glam::Vec3A;

use crate::core::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Self = Interval {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    #[allow(dead_code)]
    pub const UNIVERSE: Self = Interval {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    pub fn new(min: f32, max: f32) -> Self {
        let min = f32::min(min, max);
        let max = f32::max(min, max);
        Self { min, max }
    }

    pub fn join(a: Interval, b: Interval) -> Self {
        Self {
            min: f32::min(a.min, b.min),
            max: f32::max(a.max, b.max),
        }
    }

    pub fn size(self) -> f32 {
        self.max - self.min
    }

    #[allow(dead_code)]
    pub fn expand(self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl BoundingBox {
    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Self = Self {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn new(a: Vec3A, b: Vec3A) -> Self {
        Self {
            x: Interval::new(a.x, b.x),
            y: Interval::new(a.y, b.y),
            z: Interval::new(a.z, b.z),
        }
    }

    pub fn join(a: BoundingBox, b: BoundingBox) -> Self {
        Self {
            x: Interval::join(a.x, b.x),
            y: Interval::join(a.y, b.y),
            z: Interval::join(a.z, b.z),
        }
    }

    pub fn expand(self, delta: f32) -> Self {
        Self {
            x: self.x.expand(delta),
            y: self.y.expand(delta),
            z: self.z.expand(delta),
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    pub fn hit(&self, ray: Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let ax = self[axis];
            let adinv = 1.0 / ray.direction[axis];

            let t0 = (ax.min - ray.origin[axis]) * adinv;
            let t1 = (ax.max - ray.origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}

impl Index<usize> for BoundingBox {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}
