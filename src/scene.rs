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
    pub fn intersect(&self, ray: &mut Ray) -> Option<Surfel> {
        self.primitives.intersect(ray)
    }

    pub fn is_occluded(&self, ray: &mut Ray) -> bool {
        self.primitives.intersect_any(ray)
    }

    pub fn get_material(&self, index: usize) -> Option<&Material> {
        self.materials.get(index)
    }
}
