use derive_more::From;

use crate::{
    RGBColor,
    camera::Camera,
    error::Result,
    material::Material,
    math::{Point2, Vec3},
    ray::Ray,
    scene::Scene,
};

#[derive(From)]
pub enum Integrator {
    Sampler(SamplerIntegrator),
}

impl Integrator {
    pub fn render(&mut self) -> Result<()> {
        match self {
            Integrator::Sampler(inner) => inner.render(),
        }
    }
}

impl From<RayCastIntegrator> for Integrator {
    fn from(value: RayCastIntegrator) -> Self {
        Integrator::from(SamplerIntegrator::from(value))
    }
}

impl From<NormalMapIntegrator> for Integrator {
    fn from(value: NormalMapIntegrator) -> Self {
        Integrator::from(SamplerIntegrator::from(value))
    }
}

#[derive(From)]
pub enum SamplerIntegrator {
    RayCast(RayCastIntegrator),
    NormalMap(NormalMapIntegrator),
}

impl SamplerIntegrator {
    pub fn render(&mut self) -> Result<()> {
        self.preprocess();

        let height = self.camera().film().height();
        let width = self.camera().film().width();

        for row in 0..height {
            let normalized_row = row as f64 / (height - 1) as f64;

            for col in 0..width {
                let normalized_col = col as f64 / (width - 1) as f64;

                let ray = self.camera().generate_ray(Point2 { row, col });

                let color = self.li(ray).unwrap_or_else(|| {
                    self.scene()
                        .background
                        .sample(normalized_row, normalized_col)
                });

                self.camera().film().add_sample(Point2 { row, col }, color);
            }
        }

        self.camera().film().write_image()?;

        Ok(())
    }

    fn li(&self, ray: Ray) -> Option<RGBColor> {
        match self {
            SamplerIntegrator::RayCast(inner) => inner.li(ray),
            SamplerIntegrator::NormalMap(inner) => inner.li(ray),
        }
    }

    fn preprocess(&mut self) {
        match self {
            SamplerIntegrator::RayCast(_) => (),
            SamplerIntegrator::NormalMap(_) => (),
        }
    }

    fn camera(&mut self) -> &mut Camera {
        match self {
            SamplerIntegrator::RayCast(inner) => &mut inner.camera,
            SamplerIntegrator::NormalMap(inner) => &mut inner.camera,
        }
    }

    fn scene(&self) -> &Scene {
        match self {
            SamplerIntegrator::RayCast(inner) => &inner.scene,
            SamplerIntegrator::NormalMap(inner) => &inner.scene,
        }
    }
}

pub struct RayCastIntegrator {
    camera: Camera,
    scene: Scene,
}

impl RayCastIntegrator {
    pub fn new(camera: Camera, scene: Scene) -> Self {
        Self { camera, scene }
    }

    fn li(&self, ray: Ray) -> Option<RGBColor> {
        let isect = self.scene.intersect(ray)?;

        let material = self.scene.get_material(isect.material_id);

        material.map(|&Material::Flat { kd }| kd)
    }
}

pub struct NormalMapIntegrator {
    camera: Camera,
    scene: Scene,
}

impl NormalMapIntegrator {
    pub fn new(camera: Camera, scene: Scene) -> Self {
        Self { camera, scene }
    }

    pub fn li(&self, ray: Ray) -> Option<RGBColor> {
        let isect = self.scene.intersect(ray)?;

        let normal = isect.n
            + Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            } / 2.0;

        Some(RGBColor {
            red: (normal.x * 255.0) as u8,
            green: (normal.y * 255.0) as u8,
            blue: (normal.z * 255.0) as u8,
        })
    }
}
