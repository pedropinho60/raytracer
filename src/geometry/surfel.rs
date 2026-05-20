use glam::Vec3A;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub u: f32,
    pub v: f32,
    pub t: f32,
}

#[derive(Clone, Copy)]
pub struct Surfel {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub material_id: usize,
}
