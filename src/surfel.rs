use glam::Vec3A;

#[derive(Clone, Copy)]
pub struct Surfel {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub from_behind: bool,
    pub material_id: usize,
}
