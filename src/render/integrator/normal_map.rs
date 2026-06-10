use glam::Vec3A;

use crate::{
    core::{color::Color, ray::Ray},
    render::integrator::{Integrator, SamplerIntegrator},
    scene::Scene,
};

pub struct NormalMapIntegrator;

impl NormalMapIntegrator {
    pub fn li(ray: &mut Ray, scene: &Scene) -> Option<Color> {
        let isect = scene.intersect(ray)?;

        let normal = (isect.normal + Vec3A::ONE) / 2.0;

        Some(Color {
            red: normal.x,
            green: normal.y,
            blue: normal.z,
        })
    }
}

impl From<NormalMapIntegrator> for Integrator {
    fn from(value: NormalMapIntegrator) -> Self {
        Integrator::from(SamplerIntegrator::from(value))
    }
}
