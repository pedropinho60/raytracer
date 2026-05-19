use crate::{
    core::{color::Color, ray::Ray},
    render::integrator::{Integrator, SamplerIntegrator},
    scene::{Scene, light::Light, material::Material},
};

pub struct ToonIntegrator {
    cos_thresholds: Vec<f32>,
}

impl ToonIntegrator {
    pub fn new(mapping_interval: &[u8]) -> Self {
        let cos_thresholds = mapping_interval
            .iter()
            .map(|&angle| f32::from(angle).to_radians().cos())
            .collect();

        Self { cos_thresholds }
    }

    pub fn li(&self, ray: Ray, scene: &Scene) -> Option<Color> {
        let isect = scene.intersect(ray)?;

        let material = scene.get_material(isect.material_id).unwrap();

        let Material::Toon(material) = material else {
            return None;
        };

        let n = isect.normal.normalize();

        let v = -ray.direction.normalize();
        let ndotv = f32::max(0.0, n.dot(v));

        let edge_threshold = 0.2;

        if ndotv < edge_threshold {
            return Some(Color::BLACK);
        }

        let mut color = Color::default();

        'outer: for light in scene.lights {
            let (l, _i) = match light {
                Light::Ambient(ambient_light) => {
                    color += ambient_light.intensity * material.ambient;
                    continue;
                }
                Light::Point(point_light) => {
                    let direction = point_light.point - isect.point;
                    let distance = direction.dot(direction).sqrt();

                    let l = direction.normalize();
                    let intensity = point_light.intensity * point_light.attenuation(distance);

                    (l, intensity)
                }
                Light::Directional(directional_light) => {
                    let l = -directional_light.direction;

                    (l, directional_light.intensity)
                }
                Light::Spotlight(spotlight) => {
                    let direction = spotlight.point - isect.point;

                    let l = direction.normalize();

                    let theta = l.dot(-spotlight.direction);

                    if theta < spotlight.cutoff_cos {
                        continue;
                    }

                    let intensity = if theta < spotlight.falloff_cos {
                        let epsilon = spotlight.falloff_cos - spotlight.cutoff_cos;
                        let t = (theta - spotlight.cutoff_cos) / epsilon;

                        Color::lerp(Color::BLACK, spotlight.intensity, t)
                    } else {
                        spotlight.intensity
                    };

                    (l, intensity)
                }
            };

            let h = n.dot(l);

            for (i, &cos) in self.cos_thresholds.iter().enumerate() {
                if h > cos {
                    let color_idx = material.color_map.len().saturating_sub(1).saturating_sub(i);

                    color = *material
                        .color_map
                        .get(color_idx)
                        .or_else(|| material.color_map.last())?;
                    continue 'outer;
                }
            }

            color = *material.color_map.first()?;
        }

        Some(color)
    }
}

impl From<ToonIntegrator> for Integrator {
    fn from(value: ToonIntegrator) -> Self {
        Integrator::from(SamplerIntegrator::from(value))
    }
}
