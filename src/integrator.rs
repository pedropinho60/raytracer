use derive_more::From;
use glam::Vec3A;

use crate::{
    camera::Camera, color::Color, error::Result, film::Film, light::Light, material::Material,
    ray::Ray, scene::Scene,
};

use rayon::prelude::*;

#[derive(From)]
pub enum Integrator {
    Sampler(SamplerIntegrator),
}

impl Integrator {
    pub fn render(&mut self, camera: &mut Camera, scene: &Scene, film: &mut Film) -> Result<()> {
        match self {
            Integrator::Sampler(inner) => inner.render(camera, scene, film),
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

impl From<BlinnPhongIntegrator> for Integrator {
    fn from(value: BlinnPhongIntegrator) -> Self {
        Integrator::from(SamplerIntegrator::from(value))
    }
}

#[derive(From)]
pub enum SamplerIntegrator {
    RayCast(RayCastIntegrator),
    NormalMap(NormalMapIntegrator),
    BlinnPhong(BlinnPhongIntegrator),
}

impl SamplerIntegrator {
    pub fn render(&mut self, camera: &mut Camera, scene: &Scene, film: &mut Film) -> Result<()> {
        self.preprocess();

        let width = film.width();
        let height = film.height();

        let pixels = film.pixels_mut();

        pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, color)| {
                let col = index % width as usize;
                let row = index / width as usize;

                let ray = camera.generate_ray(row as u16, col as u16, width, height);

                let normalized_row = row as f32 / (height - 1) as f32;
                let normalized_col = col as f32 / (width - 1) as f32;

                *color = self
                    .li(ray, scene)
                    .unwrap_or_else(|| scene.background.sample(normalized_row, normalized_col));
            });

        film.write_image()?;

        Ok(())
    }

    fn li(&self, ray: Ray, scene: &Scene) -> Option<Color> {
        match self {
            SamplerIntegrator::RayCast(inner) => inner.li(ray, scene),
            SamplerIntegrator::NormalMap(inner) => inner.li(ray, scene),
            SamplerIntegrator::BlinnPhong(inner) => inner.li(ray, scene),
        }
    }

    fn preprocess(&mut self) {
        match self {
            SamplerIntegrator::RayCast(_) => (),
            SamplerIntegrator::NormalMap(_) => (),
            SamplerIntegrator::BlinnPhong(_) => (),
        }
    }
}

pub struct RayCastIntegrator;

impl RayCastIntegrator {
    fn li(&self, ray: Ray, scene: &Scene) -> Option<Color> {
        let isect = scene.intersect(ray)?;

        let material = scene.get_material(isect.material_id)?;

        match material {
            Material::Flat { kd } => Some(*kd),
            Material::Checkerboard(inner) => Some(inner.color_at(isect.point)),
            _ => None,
        }
    }
}

pub struct NormalMapIntegrator;

impl NormalMapIntegrator {
    pub fn li(&self, ray: Ray, scene: &Scene) -> Option<Color> {
        let isect = scene.intersect(ray)?;

        let mut normal = if isect.from_behind {
            isect.normal
        } else {
            -isect.normal
        };

        normal += Vec3A::new(1.0, 1.0, 1.0) / 2.0;

        Some(Color {
            red: normal.x,
            green: normal.y,
            blue: normal.z,
        })
    }
}

pub struct BlinnPhongIntegrator;

impl BlinnPhongIntegrator {
    pub fn li(&self, ray: Ray, scene: &Scene) -> Option<Color> {
        let isect = scene.intersect(ray)?;

        if isect.from_behind {
            return None;
        }

        let material = scene.get_material(isect.material_id)?;

        let Material::BlinnPhong(m) = material else {
            return None;
        };

        let kd = m.diffuse;
        let ks = m.specular;
        let g = m.glossiness;
        let ka = m.ambient;

        let v = -ray.direction.normalize();
        let n = isect.normal.normalize();

        let mut color = Color::default();

        for light in &scene.lights {
            let (l, i) = match light {
                Light::Ambient(ambient_light) => {
                    color += ambient_light.intensity * ka;
                    continue;
                }
                Light::Point(point_light) => {
                    let direction = point_light.point - isect.point;
                    let distance = direction.dot(direction).sqrt();

                    let l = (point_light.point - isect.point).normalize();
                    let intensity = point_light.intensity * point_light.attenuation(distance);

                    (l, intensity)
                }
                Light::Directional(directional_light) => {
                    (-directional_light.direction, directional_light.intensity)
                }
            };

            let h = (v + l).normalize();

            let diffuse_term = i * kd * f32::max(n.dot(l), 0.0);
            let specular_term = i * ks * f32::max(n.dot(h), 0.0).powf(g);

            color += diffuse_term + specular_term;
        }

        Some(color)
    }
}
