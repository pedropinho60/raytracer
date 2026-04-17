use crate::{
    background::Background, material::Material, primitive::AggregatePrimitive, ray::Ray,
    surfel::Surfel,
};

pub struct Scene {
    pub background: Background,
    pub materials: Vec<Material>,
    pub primitives: AggregatePrimitive,
}

impl Scene {
    pub fn intersect(&self, ray: Ray) -> Option<Surfel> {
        self.primitives.intersect(ray)
    }

    pub fn get_material(&self, index: usize) -> Option<&Material> {
        self.materials.get(index)
    }
}
