use glam::Vec3A;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub inv_dir: Vec3A,
    pub t_min: f32,
    pub t_max: f32,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self {
            origin,
            direction,
            inv_dir: direction.recip(),
            t_min: 0.0001,
            t_max: f32::INFINITY,
        }
    }
}
