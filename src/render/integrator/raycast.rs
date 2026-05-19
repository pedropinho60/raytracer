use crate::{
    core::{color::Color, ray::Ray},
    render::integrator::{Integrator, SamplerIntegrator},
    scene::{Scene, material::Material},
};

pub struct RayCastIntegrator;

impl RayCastIntegrator {
    pub fn li(ray: Ray, scene: &Scene) -> Option<Color> {
        let isect = scene.intersect(ray)?;

        let material = scene.get_material(isect.material_id)?;

        match material {
            Material::Flat { kd } => Some(*kd),
            Material::Checkerboard(inner) => Some(inner.color_at(isect.point)),
            _ => None,
        }
    }
}

impl From<RayCastIntegrator> for Integrator {
    fn from(value: RayCastIntegrator) -> Self {
        Integrator::from(SamplerIntegrator::from(value))
    }
}
