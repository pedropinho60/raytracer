use crate::math::{Point3, Vec3};

pub struct Surfel {
    pub point: Point3,
    pub normal: Vec3,
    pub from_behind: bool,
    pub material_id: usize,
}
