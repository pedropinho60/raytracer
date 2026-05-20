use crate::{
    core::ray::Ray,
    geometry::{hittable::Hit, surfel::Surfel},
    render::aggregator::PrimitiveAggregator,
    scene::{background::Background, light::Light, material::Material},
};

pub mod background;
pub mod camera;
pub mod light;
pub mod material;

pub struct Scene<'a> {
    pub background: &'a Background,
    pub materials: &'a [Material],
    pub primitives: &'a PrimitiveAggregator,
    pub lights: &'a [Light],
}

impl Scene<'_> {
    pub fn intersect(&self, ray: Ray) -> Option<Surfel> {
        self.primitives.intersect(ray, 0.001, f32::INFINITY)
    }

    pub fn is_occluded(&self, ray: Ray, distance: f32) -> bool {
        self.primitives.intersect_any(ray, 0.001, distance)
    }

    pub fn get_material(&self, index: usize) -> Option<&Material> {
        self.materials.get(index)
    }
}
