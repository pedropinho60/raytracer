mod blinn_phong;
mod celshading;
mod normal_map;
mod raycast;

use derive_more::From;
use rayon::prelude::*;

pub use blinn_phong::BlinnPhongIntegrator;
pub use celshading::CelShadingIntegrator;
pub use normal_map::NormalMapIntegrator;
pub use raycast::RayCastIntegrator;

use crate::{
    core::{color::Color, ray::Ray},
    parse::dto::IntegratorDTO,
    render::film::Film,
    scene::{Scene, camera::Camera},
};

#[derive(From)]
pub enum Integrator {
    Sampler(SamplerIntegrator),
}

impl Integrator {
    pub fn render(&mut self, camera: &mut Camera, scene: &Scene, film: &mut Film) {
        match self {
            Integrator::Sampler(inner) => inner.render(camera, scene, film),
        }
    }
}

impl From<&IntegratorDTO> for Integrator {
    fn from(value: &IntegratorDTO) -> Self {
        match value {
            IntegratorDTO::Flat => RayCastIntegrator.into(),
            IntegratorDTO::NormalMap => NormalMapIntegrator.into(),
            IntegratorDTO::BlinnPhong { depth } => BlinnPhongIntegrator::new(*depth).into(),
            IntegratorDTO::CelShading { mapping_interval } => {
                CelShadingIntegrator::new(&mapping_interval.0).into()
            }
        }
    }
}

#[derive(From)]
pub enum SamplerIntegrator {
    RayCast(RayCastIntegrator),
    NormalMap(NormalMapIntegrator),
    BlinnPhong(BlinnPhongIntegrator),
    CelShading(CelShadingIntegrator),
}

impl SamplerIntegrator {
    pub fn render(&mut self, camera: &mut Camera, scene: &Scene, film: &mut Film) {
        self.preprocess();

        let width = film.width() as usize;
        let height = film.height() as usize;

        let inv_width = (width - 1) as f32;
        let inv_height = (height - 1) as f32;

        let pixels = film.pixels_mut();

        pixels
            .par_chunks_exact_mut(width)
            .enumerate()
            .for_each(|(row, buf)| {
                let normalized_row = row as f32 / inv_height;
                for (col, pixel) in buf.iter_mut().enumerate() {
                    let mut ray = camera.generate_ray(row, col, width, height);

                    let normalized_col = col as f32 / inv_width;

                    *pixel = self
                        .li(&mut ray, scene)
                        .unwrap_or_else(|| scene.background.sample(normalized_row, normalized_col));
                }
            });
    }

    fn li(&self, ray: &mut Ray, scene: &Scene) -> Option<Color> {
        match self {
            SamplerIntegrator::RayCast(_) => RayCastIntegrator::li(ray, scene),
            SamplerIntegrator::NormalMap(_) => NormalMapIntegrator::li(ray, scene),
            SamplerIntegrator::BlinnPhong(inner) => inner.li(ray, scene, 0),
            SamplerIntegrator::CelShading(inner) => inner.li(ray, scene),
        }
    }

    fn preprocess(&mut self) {
        match self {
            SamplerIntegrator::RayCast(_)
            | SamplerIntegrator::NormalMap(_)
            | SamplerIntegrator::BlinnPhong(_)
            | SamplerIntegrator::CelShading(_) => (),
        }
    }
}
