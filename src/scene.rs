use crate::{
    aggregator::PrimitiveAggregator, background::Background, hittable::Hit, light::Light,
    material::Material, ray::Ray, surfel::Surfel,
};

pub struct Scene<'a> {
    pub background: &'a Background,
    pub materials: &'a [Material],
    pub primitives: &'a PrimitiveAggregator,
    pub lights: &'a [Light],
}

impl<'a> Scene<'a> {
    pub fn intersect(&self, ray: Ray) -> Option<Surfel> {
        self.primitives
            .intersect(ray, 0.001, f32::INFINITY)
            .map(|(_, s)| s)
    }

    pub fn is_occluded(&self, ray: Ray, distance: f32) -> bool {
        self.primitives.intersect_any(ray, 0.001, distance)
    }

    pub fn get_material(&self, index: usize) -> Option<&Material> {
        self.materials.get(index)
    }
}
