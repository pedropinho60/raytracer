use derive_more::From;

use crate::{
    camera::Camera,
    color::Color,
    error::Result,
    light::Light,
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

    fn li(&self, ray: Ray) -> Option<Color> {
        match self {
            SamplerIntegrator::RayCast(inner) => inner.li(ray),
            SamplerIntegrator::NormalMap(inner) => inner.li(ray),
            SamplerIntegrator::BlinnPhong(inner) => inner.li(ray),
        }
    }

    fn preprocess(&mut self) {
        match self {
            SamplerIntegrator::RayCast(_) => (),
            SamplerIntegrator::NormalMap(_) => (),
            SamplerIntegrator::BlinnPhong(_) => (),
        }
    }

    fn camera(&mut self) -> &mut Camera {
        match self {
            SamplerIntegrator::RayCast(inner) => &mut inner.camera,
            SamplerIntegrator::NormalMap(inner) => &mut inner.camera,
            SamplerIntegrator::BlinnPhong(inner) => &mut inner.camera,
        }
    }

    fn scene(&self) -> &Scene {
        match self {
            SamplerIntegrator::RayCast(inner) => &inner.scene,
            SamplerIntegrator::NormalMap(inner) => &inner.scene,
            SamplerIntegrator::BlinnPhong(inner) => &inner.scene,
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

    fn li(&self, ray: Ray) -> Option<Color> {
        let isect = self.scene.intersect(ray)?;

        let material = self.scene.get_material(isect.material_id)?;

        match material {
            Material::Flat { kd } => Some(*kd),
            Material::Checkerboard(inner) => Some(inner.color_at(isect.point)),
            _ => None,
        }
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

    pub fn li(&self, ray: Ray) -> Option<Color> {
        let isect = self.scene.intersect(ray)?;

        let mut normal = if isect.from_behind {
            isect.normal
        } else {
            -isect.normal
        };

        normal += Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        } / 2.0;

        Some(Color {
            red: normal.x,
            green: normal.y,
            blue: normal.z,
        })
    }
}

pub struct BlinnPhongIntegrator {
    camera: Camera,
    scene: Scene,
}

impl BlinnPhongIntegrator {
    pub fn new(camera: Camera, scene: Scene) -> Self {
        Self { camera, scene }
    }

    pub fn li(&self, ray: Ray) -> Option<Color> {
        let isect = self.scene.intersect(ray)?;

        if isect.from_behind {
            return None;
        }

        let material = self.scene.get_material(isect.material_id)?;

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

        for light in &self.scene.lights {
            let (l, i) = match light {
                Light::Ambient(ambient_light) => {
                    color += ambient_light.intensity * ka;
                    continue;
                }
                Light::Point(point_light) => (
                    (point_light.point - isect.point).normalize(),
                    point_light.intensity,
                ),
                Light::Directional(directional_light) => {
                    (-directional_light.direction, directional_light.intensity)
                }
            };

            let h = (v + l).normalize();

            let diffuse_term = i * kd * f64::max(n.dot(l), 0.0);
            let specular_term = i * ks * f64::max(n.dot(h), 0.0).powf(g);

            color += diffuse_term + specular_term;
        }

        Some(color)
    }
}
