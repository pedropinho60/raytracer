use derive_more::From;

use crate::{
    color::Color,
    math::{Point3, Vec3},
};

#[derive(Clone, From)]
pub enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
    Ambient(AmbientLight),
}

#[derive(Clone)]
pub struct PointLight {
    pub intensity: Color,
    pub point: Point3,
}

#[derive(Clone)]
pub struct DirectionalLight {
    pub intensity: Color,
    pub direction: Vec3,
}

#[derive(Clone)]
pub struct AmbientLight {
    pub intensity: Color,
}
