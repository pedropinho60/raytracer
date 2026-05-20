use crate::{
    core::{color::Color, ray::Ray},
    render::integrator::{Integrator, SamplerIntegrator},
    scene::{Scene, light::Light, material::Material},
};

pub struct BlinnPhongIntegrator {
    max_depth: u8,
}

impl BlinnPhongIntegrator {
    pub fn new(depth: u8) -> Self {
        Self { max_depth: depth }
    }

    pub fn li(&self, ray: Ray, scene: &Scene, depth: u8) -> Option<Color> {
        let isect = scene.intersect(ray)?;

        let material = scene.get_material(isect.material_id).unwrap();

        let (kd, ks, g, ka, km) = match material {
            Material::Flat { kd } => (*kd, Color::BLACK, 0, Color::BLACK, Color::BLACK),
            Material::Checkerboard(m) => (
                m.color_at(isect.u, isect.v),
                Color::BLACK,
                0,
                Color {
                    red: 0.1,
                    green: 0.1,
                    blue: 0.1,
                },
                Color::BLACK,
            ),
            Material::BlinnPhong(m) => (m.diffuse, m.specular, m.glossiness, m.ambient, m.mirror),
            Material::Toon(_) => return None,
        };

        let v = -ray.direction.normalize();
        let n = isect.normal.normalize();

        let mut color = Color::default();

        for light in scene.lights {
            let (l, i) = match light {
                Light::Ambient(ambient_light) => {
                    color += ambient_light.intensity * ka;
                    continue;
                }
                Light::Point(point_light) => {
                    let direction = point_light.point - isect.point;
                    let distance = direction.dot(direction).sqrt();

                    let l = direction.normalize();
                    let intensity = point_light.intensity * point_light.attenuation(distance);

                    let shadow_ray = Ray {
                        origin: isect.point,
                        direction: l,
                    };

                    if scene.is_occluded(shadow_ray, distance) {
                        continue;
                    }

                    (l, intensity)
                }
                Light::Directional(directional_light) => {
                    let l = -directional_light.direction;

                    let shadow_ray = Ray {
                        origin: isect.point,
                        direction: l,
                    };

                    if scene.is_occluded(shadow_ray, f32::INFINITY) {
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
                        continue;
                    }

                    let intensity = if theta < spotlight.falloff_cos {
                        let epsilon = spotlight.falloff_cos - spotlight.cutoff_cos;
                        let t = (theta - spotlight.cutoff_cos) / epsilon;

                        Color::lerp(Color::BLACK, spotlight.intensity, t)
                    } else {
                        spotlight.intensity
                    };

                    let shadow_ray = Ray {
                        origin: isect.point,
                        direction: l,
                    };

                    if scene.is_occluded(shadow_ray, distance) {
                        continue;
                    }

                    (l, intensity)
                }
            };

            let h = (v + l).normalize();

            let diffuse_term = i * kd * f32::max(n.dot(l), 0.0);
            let specular_term = if g > 0 {
                i * ks * f32::max(n.dot(h), 0.0).powi(g.into())
            } else {
                Color::BLACK
            };

            color += diffuse_term + specular_term;
        }

        let reflected_ray = Ray {
            origin: isect.point,
            direction: ray.direction - 2.0 * ray.direction.dot(n) * n,
        };

        if depth < self.max_depth && km != Color::BLACK {
            color += km
                * self
                    .li(reflected_ray, scene, depth + 1)
                    .unwrap_or_else(|| scene.background.sample_ray(reflected_ray));
        }

        Some(color)
    }
}

impl From<BlinnPhongIntegrator> for Integrator {
    fn from(value: BlinnPhongIntegrator) -> Self {
        Integrator::from(SamplerIntegrator::from(value))
    }
}
