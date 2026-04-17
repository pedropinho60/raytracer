use crate::math::{Point3, Vec3};

pub struct Surfel {
    pub p: Point3,
    pub n: Vec3,
    pub material_id: usize,
}
