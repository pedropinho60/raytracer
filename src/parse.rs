use std::{fmt::Display, path::PathBuf, str::FromStr};

use glam::Vec3A;
use serde::{Deserialize, Deserializer};

use crate::{
    WindowSize,
    aggregator::{PrimitiveAggregator, PrimitiveBVH, PrimitiveList},
    background::{Background, GradientBackground, SingleColorBackground},
    camera::{Camera, OrthographicCamera, PerspectiveCamera},
    color::{Color, ColorU8},
    dithering::{BayerDithering, BlueNoiseDithering, Dithering, WhiteNoiseDithering},
    film::ImageType,
    hittable::Hittable,
    integrator::{
        BlinnPhongIntegrator, Integrator, NormalMapIntegrator, RayCastIntegrator, ToonIntegrator,
    },
    light::{AmbientLight, Attenuation, DirectionalLight, Light, PointLight, Spotlight},
    material::{BlinnPhongMaterial, CheckerboardMaterial, Material, ToonMaterial},
    primitive::{Plane, Primitive, Sphere},
};

fn parse_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
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
    Aggregator(AggregatorType),
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
    pub look_from: Vec3String,
    #[serde(rename = "@look_at")]
    pub look_at: Vec3String,
    #[serde(rename = "@up")]
    pub up: Vec3String,
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
        #[serde(rename = "@fovy", deserialize_with = "parse_from_string")]
        fovy: u16,
    },
}

impl CameraType {
    pub fn to_camera(self, camera_args: CameraArgs, width: u16, height: u16) -> Camera {
        let CameraArgs {
            look_from,
            look_at,
            up,
        } = camera_args;

        match self {
            CameraType::Orthographic { screen_window } => {
                OrthographicCamera::new(look_from.into(), look_at.into(), up.into(), screen_window)
                    .into()
            }
            CameraType::Perspective { fovy } => PerspectiveCamera::new(
                look_from.into(),
                look_at.into(),
                up.into(),
                fovy,
                width,
                height,
            )
            .into(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum FilmType {
    Image {
        #[serde(
            rename = "@w_res",
            alias = "@x_res",
            deserialize_with = "parse_from_string"
        )]
        width: u16,
        #[serde(
            rename = "@h_res",
            alias = "@y_res",
            deserialize_with = "parse_from_string"
        )]
        height: u16,
        #[serde(rename = "@filename")]
        filename: PathBuf,
        #[serde(rename = "@img_type")]
        img_type: ImageType,
        #[serde(
            rename = "@gamma_corrected",
            default,
            deserialize_with = "parse_from_string"
        )]
        gamma_corrected: bool,
        #[serde(rename = "@dithering", default)]
        dithering: DitheringType,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DitheringType {
    #[default]
    None,
    Bayer,
    WhiteNoise,
    BlueNoise,
}

impl DitheringType {
    pub fn to_dithering(self) -> Dithering {
        match self {
            DitheringType::None => Dithering::None,
            DitheringType::Bayer => BayerDithering.into(),
            DitheringType::WhiteNoise => WhiteNoiseDithering.into(),
            DitheringType::BlueNoise => BlueNoiseDithering::new().into(),
        }
    }
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
            BackgroundType::SingleColor { color } => SingleColorBackground {
                color: color.into(),
            }
            .into(),
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
        from: Vec3String,
        #[serde(rename = "@attenuation")]
        attenuation: Option<Attenuation>,
    },
    Directional {
        #[serde(rename = "@I")]
        intensity: Color,
        #[serde(rename = "@scale")]
        scale: Color,
        #[serde(rename = "@from")]
        from: Vec3String,
        #[serde(rename = "@to")]
        to: Vec3String,
    },
    #[serde(rename = "spot")]
    Spotlight {
        #[serde(rename = "@I")]
        intensity: Color,
        #[serde(rename = "@from")]
        from: Vec3String,
        #[serde(rename = "@to")]
        to: Vec3String,
        #[serde(rename = "@cutoff", deserialize_with = "parse_from_string")]
        cutoff: f32,
        #[serde(rename = "@falloff", deserialize_with = "parse_from_string")]
        falloff: f32,
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
                attenuation,
            } => PointLight {
                intensity: intensity * scale,
                point: from.into(),
                attenuation: attenuation.unwrap_or_default(),
            }
            .into(),
            LightType::Directional {
                intensity,
                scale,
                from,
                to,
            } => {
                let to: Vec3A = to.into();
                let from: Vec3A = from.into();
                DirectionalLight {
                    intensity: intensity * scale,
                    direction: (to - from).normalize(),
                }
                .into()
            }
            LightType::Spotlight {
                intensity,
                from,
                to,
                cutoff,
                falloff,
            } => {
                let to: Vec3A = to.into();
                let from: Vec3A = from.into();
                Spotlight {
                    intensity,
                    point: from,
                    direction: (to - from).normalize(),
                    cutoff_cos: cutoff.to_radians().cos(),
                    falloff_cos: falloff.to_radians().cos(),
                }
                .into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Sphere {
        #[serde(rename = "@center")]
        center: Vec3String,
        #[serde(rename = "@radius", deserialize_with = "parse_from_string")]
        radius: f32,
    },
    Plane {
        #[serde(rename = "@point")]
        point: Vec3String,
        #[serde(rename = "@normal")]
        normal: Vec3String,
    },
}

impl ObjectType {
    pub fn to_primitive(self, material_id: usize) -> Primitive {
        match self {
            ObjectType::Sphere { center, radius } => Primitive::new(
                Sphere {
                    center: center.into(),
                    radius,
                }
                .into(),
                material_id,
            ),
            ObjectType::Plane { point, normal } => {
                Primitive::new(Plane::new(point.into(), normal.into()).into(), material_id)
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
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
        #[serde(rename = "@scale", deserialize_with = "parse_from_string")]
        scale: f32,
    },
    Blinn {
        #[serde(rename = "@ambient")]
        ambient: Color,
        #[serde(rename = "@diffuse")]
        diffuse: Color,
        #[serde(rename = "@specular")]
        specular: Color,
        #[serde(rename = "@glossiness", deserialize_with = "parse_from_string")]
        glossiness: u16,
        #[serde(rename = "@mirror", default)]
        mirror: Color,
    },
    Toon {
        #[serde(rename = "@color_map")]
        color_map: Colors,
    },
}

impl MaterialType {
    pub fn into_material(self) -> Material {
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
                mirror,
            } => BlinnPhongMaterial::new(diffuse, specular, glossiness, ambient, mirror).into(),
            MaterialType::Toon { color_map } => ToonMaterial::new(color_map.0, Color::BLACK).into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum IntegratorType {
    Flat,
    NormalMap,
    BlinnPhong {
        #[serde(rename = "@depth", deserialize_with = "parse_from_string")]
        depth: u8,
    },
    Toon {
        #[serde(rename = "@mapping_interval")]
        mapping_interval: ArrayString<u8>,
    },
}

impl IntegratorType {
    pub fn to_integrator(&self) -> Integrator {
        match self {
            IntegratorType::Flat => RayCastIntegrator.into(),
            IntegratorType::NormalMap => NormalMapIntegrator.into(),
            IntegratorType::BlinnPhong { depth } => BlinnPhongIntegrator::new(*depth).into(),
            IntegratorType::Toon { mapping_interval } => {
                ToonIntegrator::new(&mapping_interval.0).into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum AggregatorType {
    List,
    Tree,
}

impl AggregatorType {
    pub fn to_aggregator(self, list: &[Hittable]) -> PrimitiveAggregator {
        match self {
            AggregatorType::List => PrimitiveList::new(list).into(),
            AggregatorType::Tree => PrimitiveBVH::new(list).into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3String {
    x: f32,
    y: f32,
    z: f32,
}

impl<'de> Deserialize<'de> for Vec3String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(serde::de::Error::custom(
                "Expected exactly 3 coordinate components",
            ));
        }

        let x = parts[0].parse().map_err(serde::de::Error::custom)?;
        let y = parts[1].parse().map_err(serde::de::Error::custom)?;
        let z = parts[2].parse().map_err(serde::de::Error::custom)?;

        Ok(Vec3String { x, y, z })
    }
}

impl From<Vec3String> for Vec3A {
    fn from(value: Vec3String) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[derive(Debug, Clone)]
pub struct Colors(pub Vec<Color>);

impl<'de> Deserialize<'de> for Colors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() % 3 != 0 {
            return Err(serde::de::Error::custom(
                "Expected a multiple of 3 color values",
            ));
        }

        let mut colors = Vec::new();

        for color in parts.chunks_exact(3) {
            let red = color[0].parse().map_err(serde::de::Error::custom)?;
            let green = color[1].parse().map_err(serde::de::Error::custom)?;
            let blue = color[2].parse().map_err(serde::de::Error::custom)?;

            colors.push(ColorU8 { red, green, blue }.into())
        }

        Ok(Self(colors))
    }
}

#[derive(Debug, Clone)]
pub struct ArrayString<T>(pub Vec<T>);

impl<'de, T> Deserialize<'de> for ArrayString<T>
where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        let x = parts
            .iter()
            .map(|a| a.parse::<T>())
            .collect::<Result<Vec<T>, _>>()
            .map_err(serde::de::Error::custom)?;

        Ok(Self(x))
    }
}
