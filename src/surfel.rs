use glam::Vec3;

pub struct Surfel {
    pub point: Vec3,
    pub normal: Vec3,
    pub from_behind: bool,
    pub material_id: usize,
}
