use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Deserializer};

use crate::{
    WindowSize,
    background::{Background, GradientBackground, SingleColorBackground},
    camera::{Camera, OrthographicCamera, PerspectiveCamera},
    color::{Color, ColorU8},
    film::ImageType,
    integrator::{BlinnPhongIntegrator, Integrator, NormalMapIntegrator, RayCastIntegrator},
    light::{AmbientLight, DirectionalLight, Light, PointLight},
    material::{BlinnPhongMaterial, CheckerboardMaterial, Material},
    math::{Point3, Vec3},
    primitive::{Plane, Primitive, Sphere},
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
    Aggregator {
        #[serde(rename = "@type")]
        ty: String,
    },
    WorldBegin,
    MakeNamedMaterial {
        #[serde(rename = "@name")]
        name: String,
        #[serde(flatten)]
        material_type: MaterialType,
    },
    NamedMaterial {
        #[serde(rename = "@name")]
        name: String,
    },
    Material(MaterialType),
    Object(ObjectType),
    Background(BackgroundType),
    LightSource(LightType),
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
    pub fn to_camera(self, camera_args: CameraArgs) -> Camera {
        let CameraArgs {
            look_from,
            look_at,
            up,
        } = camera_args;

        match self {
            CameraType::Orthographic { screen_window } => {
                OrthographicCamera::new(look_from, look_at, up, screen_window).into()
            }
            CameraType::Perspective { fovy } => {
                PerspectiveCamera::new(look_from, look_at, up, fovy).into()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum FilmType {
    Image {
        #[serde(rename = "@w_res", alias = "@x_res", deserialize_with = "parse_number")]
        w_res: u16,
        #[serde(rename = "@h_res", alias = "@y_res", deserialize_with = "parse_number")]
        h_res: u16,
        #[serde(rename = "@filename")]
        filename: PathBuf,
        #[serde(rename = "@img_type")]
        img_type: ImageType,
        #[serde(
            rename = "@gamma_corrected",
            default,
            deserialize_with = "parse_number"
        )]
        gamma_corrected: bool,
    },
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum BackgroundType {
    SingleColor {
        #[serde(rename = "@color")]
        color: ColorU8,
    },
    #[serde(rename = "4_colors", alias = "colors")]
    FourColors {
        #[serde(rename = "@bl")]
        bl: ColorU8,
        #[serde(rename = "@tl")]
        tl: ColorU8,
        #[serde(rename = "@tr")]
        tr: ColorU8,
        #[serde(rename = "@br")]
        br: ColorU8,
    },
}

impl BackgroundType {
    pub fn to_background(self) -> Background {
        match self {
            BackgroundType::SingleColor { color } => {
                SingleColorBackground::new(color.into()).into()
            }
            BackgroundType::FourColors { bl, tl, tr, br } => {
                GradientBackground::new(tl.into(), tr.into(), bl.into(), br.into()).into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum LightType {
    Ambient {
        #[serde(rename = "@I")]
        intensity: Color,
        #[serde(rename = "@scale")]
        scale: Color,
    },
    Point {
        #[serde(rename = "@I")]
        intensity: Color,
        #[serde(rename = "@scale")]
        scale: Color,
        #[serde(rename = "@from")]
        from: Point3,
    },
    Directional {
        #[serde(rename = "@I")]
        intensity: Color,
        #[serde(rename = "@scale")]
        scale: Color,
        #[serde(rename = "@from")]
        from: Point3,
        #[serde(rename = "@to")]
        to: Point3,
    },
}

impl LightType {
    pub fn to_light(self) -> Light {
        match self {
            LightType::Ambient { intensity, scale } => AmbientLight {
                intensity: intensity * scale,
            }
            .into(),
            LightType::Point {
                intensity,
                scale,
                from,
            } => PointLight {
                intensity: intensity * scale,
                point: from,
            }
            .into(),
            LightType::Directional {
                intensity,
                scale,
                from,
                to,
            } => DirectionalLight {
                intensity: intensity * scale,
                direction: (to - from).normalize(),
            }
            .into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Sphere {
        #[serde(rename = "@center")]
        center: Point3,
        #[serde(rename = "@radius", deserialize_with = "parse_number")]
        radius: f64,
    },
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
#[serde(rename_all = "snake_case")]
pub enum MaterialType {
    Flat {
        #[serde(rename = "@color")]
        color: ColorU8,
    },
    Checkerboard {
        #[serde(rename = "@color_a")]
        color_a: ColorU8,
        #[serde(rename = "@color_b")]
        color_b: ColorU8,
        #[serde(rename = "@scale", deserialize_with = "parse_number")]
        scale: f64,
    },
    Blinn {
        #[serde(rename = "@ambient")]
        ambient: Color,
        #[serde(rename = "@diffuse")]
        diffuse: Color,
        #[serde(rename = "@specular")]
        specular: Color,
        #[serde(rename = "@glossiness", deserialize_with = "parse_number")]
        glossiness: f64,
    },
}

impl MaterialType {
    pub fn to_material(self) -> Material {
        match self {
            MaterialType::Flat { color } => Material::Flat { kd: color.into() },
            MaterialType::Checkerboard {
                color_a,
                color_b,
                scale,
            } => CheckerboardMaterial::new(color_a.into(), color_b.into(), scale).into(),
            MaterialType::Blinn {
                ambient,
                diffuse,
                specular,
                glossiness,
            } => BlinnPhongMaterial::new(diffuse, specular, glossiness, ambient).into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum IntegratorType {
    Flat,
    NormalMap,
    BlinnPhong,
}

impl IntegratorType {
    pub fn to_integrator(self) -> Integrator {
        match self {
            IntegratorType::Flat => RayCastIntegrator.into(),
            IntegratorType::NormalMap => NormalMapIntegrator.into(),
            IntegratorType::BlinnPhong => BlinnPhongIntegrator.into(),
        }
    }
}
