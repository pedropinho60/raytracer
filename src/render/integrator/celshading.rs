use crate::{
    core::{color::Color, ray::Ray},
    render::integrator::{Integrator, SamplerIntegrator},
    scene::{Scene, light::Light, material::Material},
};

pub struct CelShadingIntegrator {
    cos_thresholds: Vec<f32>,
}

impl CelShadingIntegrator {
    pub fn new(mapping_interval: &[u8]) -> Self {
        let cos_thresholds = mapping_interval
            .iter()
            .map(|&angle| f32::from(angle).to_radians().cos())
            .collect();

        Self { cos_thresholds }
    }

    pub fn li(&self, ray: &mut Ray, scene: &Scene) -> Option<Color> {
        let isect = scene.intersect(ray)?;

        let material = scene.get_material(isect.material_id).unwrap();

        let Material::Cel(material) = material else {
            return None;
        };

        let n = isect.normal.normalize();

        if let Some(threshold) = material.silhouette_cos {
            let v = -ray.direction.normalize();
            let ndotv = f32::max(0.0, n.dot(v));

            if ndotv < threshold {
                return Some(material.silhouette_color);
            }
        }

        let mut color = Color::BLACK;

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

                    let mut shadow_ray = Ray::new(isect.point, l);
                    shadow_ray.t_max = distance;

                    if scene.is_occluded(&mut shadow_ray) {
                        color = material.shadow_color;
                        continue;
                    }

                    (l, intensity)
                }
                Light::Directional(directional_light) => {
                    let l = -directional_light.direction;

                    let mut shadow_ray = Ray::new(isect.point, l);

                    if scene.is_occluded(&mut shadow_ray) {
                        color = material.shadow_color;
                        continue;
                    }

                    (l, directional_light.intensity)
                }
                Light::Spotlight(spotlight) => {
                    let direction = spotlight.point - isect.point;
                    let distance = direction.dot(direction).sqrt();

                    let l = direction.normalize();

                    let theta = l.dot(-spotlight.direction);

                    if theta < spotlight.cutoff_cos {
                        color = material.shadow_color;
                        continue;
                    }

                    let intensity = if theta < spotlight.falloff_cos {
                        let epsilon = spotlight.falloff_cos - spotlight.cutoff_cos;
                        let t = (theta - spotlight.cutoff_cos) / epsilon;

                        Color::lerp(Color::BLACK, spotlight.intensity, t)
                    } else {
                        spotlight.intensity
                    };

                    let mut shadow_ray = Ray::new(isect.point, l);
                    shadow_ray.t_max = distance;

                    if scene.is_occluded(&mut shadow_ray) {
                        color = material.shadow_color;
                        continue;
                    }

                    (l, intensity)
                }
            };

            let h = n.dot(l);

            for (i, &cos) in self.cos_thresholds.iter().enumerate() {
                if h > cos {
                    color = *material
                        .color_map
                        .get(i)
                        .or_else(|| material.color_map.last())?;
                    continue 'outer;
                }
            }

            color = *material.color_map.last()?;
        }

        Some(color)
    }
}

impl From<CelShadingIntegrator> for Integrator {
    fn from(value: CelShadingIntegrator) -> Self {
        Integrator::from(SamplerIntegrator::from(value))
    }
}
