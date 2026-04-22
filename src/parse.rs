use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Deserializer};

use crate::{
    RGBColor, WindowSize,
    background::{Background, GradientBackground, SingleColorBackground},
    camera::{Camera, OrthographicCamera, PerspectiveCamera},
    film::{Film, ImageType},
    integrator::{Integrator, NormalMapIntegrator, RayCastIntegrator},
    material::{CheckerboardMaterial, Material},
    math::{Point3, Vec3},
    primitive::{Plane, Primitive, Sphere},
    scene::Scene,
};

fn parse_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<T>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
#[serde(rename = "RT3")]
pub struct Rt3 {
    #[serde(rename = "$value")]
    pub commands: Vec<SceneCommand>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneCommand {
    Lookat(CameraArgs),
    Camera(CameraType),
    Integrator(IntegratorType),
    Film(FilmType),
    WorldBegin,
    MakeNamedMaterial {
        #[serde(rename = "@name")]
        name: String,
        material_type: MaterialType,
    },
    NamedMaterial {
        #[serde(rename = "@name")]
        name: String,
    },
    Material(MaterialType),
    Object(ObjectType),
    Background(BackgroundType),
    WorldEnd,
    RenderAgain,
    Include {
        #[serde(rename = "@filename")]
        filename: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct CameraArgs {
    #[serde(rename = "@look_from")]
    pub look_from: Point3,
    #[serde(rename = "@look_at")]
    pub look_at: Point3,
    #[serde(rename = "@up")]
    pub up: Vec3,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
pub enum CameraType {
    #[serde(rename = "orthographic")]
    Orthographic {
        #[serde(rename = "@screen_window")]
        screen_window: WindowSize,
    },
    #[serde(rename = "perspective")]
    Perspective {
        #[serde(rename = "@fovy", deserialize_with = "parse_number")]
        fovy: u16,
    },
}

impl CameraType {
    pub fn to_camera(self, camera_args: CameraArgs, film: Film) -> Camera {
        let CameraArgs {
            look_from,
            look_at,
            up,
        } = camera_args;

        match self {
            CameraType::Orthographic { screen_window } => {
                OrthographicCamera::new(look_from, look_at, up, screen_window, film).into()
            }
            CameraType::Perspective { fovy } => {
                PerspectiveCamera::new(look_from, look_at, up, fovy, film).into()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
pub enum FilmType {
    #[serde(rename = "image")]
    Image {
        #[serde(rename = "@w_res", alias = "@x_res", deserialize_with = "parse_number")]
        w_res: u16,
        #[serde(rename = "@h_res", alias = "@y_res", deserialize_with = "parse_number")]
        h_res: u16,
        #[serde(rename = "@filename")]
        filename: PathBuf,
        #[serde(rename = "@img_type")]
        img_type: ImageType,
    },
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
pub enum BackgroundType {
    #[serde(rename = "single_color")]
    SingleColor {
        #[serde(rename = "@color")]
        color: RGBColor,
    },
    #[serde(rename = "4_colors", alias = "colors")]
    FourColors {
        #[serde(rename = "@bl")]
        bl: RGBColor,
        #[serde(rename = "@tl")]
        tl: RGBColor,
        #[serde(rename = "@tr")]
        tr: RGBColor,
        #[serde(rename = "@br")]
        br: RGBColor,
    },
}

impl BackgroundType {
    pub fn to_background(self) -> Background {
        match self {
            BackgroundType::SingleColor { color } => SingleColorBackground::new(color).into(),
            BackgroundType::FourColors { bl, tl, tr, br } => {
                GradientBackground::new(tl, tr, bl, br).into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
pub enum ObjectType {
    #[serde(rename = "sphere")]
    Sphere {
        #[serde(rename = "@center")]
        center: Point3,
        #[serde(rename = "@radius", deserialize_with = "parse_number")]
        radius: f64,
    },
    #[serde(rename = "plane")]
    Plane {
        #[serde(rename = "@point")]
        point: Point3,
        #[serde(rename = "@normal")]
        normal: Vec3,
    },
}

impl ObjectType {
    pub fn to_primitive(self, material_id: usize) -> Primitive {
        match self {
            ObjectType::Sphere { center, radius } => {
                Primitive::new(Sphere { center, radius }.into(), material_id)
            }
            ObjectType::Plane { point, normal } => {
                Primitive::new(Plane::new(point, normal).into(), material_id)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
pub enum MaterialType {
    #[serde(rename = "flat")]
    Flat {
        #[serde(rename = "@color")]
        color: RGBColor,
    },
    #[serde(rename = "checkerboard")]
    Checkerboard {
        #[serde(rename = "@color_a")]
        color_a: RGBColor,
        #[serde(rename = "@color_b")]
        color_b: RGBColor,
        #[serde(rename = "@scale", deserialize_with = "parse_number")]
        scale: f64,
    },
}

impl MaterialType {
    pub fn to_material(self) -> Material {
        match self {
            MaterialType::Flat { color } => Material::Flat { kd: color },
            MaterialType::Checkerboard {
                color_a,
                color_b,
                scale,
            } => CheckerboardMaterial::new(color_a, color_b, scale).into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
pub enum IntegratorType {
    #[serde(rename = "flat")]
    Flat,
    #[serde(rename = "normal_map")]
    NormalMap,
}

impl IntegratorType {
    pub fn to_integrator(self, camera: Camera, scene: Scene) -> Integrator {
        match self {
            IntegratorType::Flat => RayCastIntegrator::new(camera, scene).into(),
            IntegratorType::NormalMap => NormalMapIntegrator::new(camera, scene).into(),
        }
    }
}
