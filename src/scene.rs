use crate::{
    background::Background, light::Light, material::Material, primitive::AggregatePrimitive,
    ray::Ray, surfel::Surfel,
};

pub struct Scene<'a> {
    pub background: &'a Background,
    pub materials: &'a [Material],
    pub primitives: &'a AggregatePrimitive,
    pub lights: &'a [Light],
}

impl<'a> Scene<'a> {
    pub fn intersect(&self, ray: Ray) -> Option<Surfel> {
        self.primitives.intersect(ray)
    }

    pub fn get_material(&self, index: usize) -> Option<&Material> {
        self.materials.get(index)
    }
}
