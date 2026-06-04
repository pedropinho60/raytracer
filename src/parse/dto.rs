use std::{fmt::Display, path::PathBuf, str::FromStr};

use glam::{Vec2, Vec3A};
use serde::{Deserialize, Deserializer};

use crate::{
    core::color::{Color, ColorU8},
    render::film::ImageType,
    scene::{camera::ViewPlane, light::Attenuation},
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

fn parse_optional_from_string<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct CameraArgsDTO {
    #[serde(rename = "@look_from")]
    pub look_from: Vec3A,
    #[serde(rename = "@look_at")]
    pub look_at: Vec3A,
    #[serde(rename = "@up")]
    pub up: Vec3A,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
pub enum CameraDTO {
    #[serde(rename = "orthographic")]
    Orthographic {
        #[serde(rename = "@screen_window")]
        screen_window: ViewPlane,
    },
    #[serde(rename = "perspective")]
    Perspective {
        #[serde(rename = "@fovy", deserialize_with = "parse_from_string")]
        fovy: u16,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum FilmDTO {
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
        dithering: DitheringDTO,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DitheringDTO {
    #[default]
    None,
    Bayer,
    WhiteNoise,
    BlueNoise,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum BackgroundDTO {
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

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum LightDTO {
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
        from: Vec3DTO,
        #[serde(rename = "@attenuation")]
        attenuation: Option<Attenuation>,
    },
    Directional {
        #[serde(rename = "@I")]
        intensity: Color,
        #[serde(rename = "@scale")]
        scale: Color,
        #[serde(rename = "@from")]
        from: Vec3DTO,
        #[serde(rename = "@to")]
        to: Vec3DTO,
    },
    #[serde(rename = "spot")]
    Spotlight {
        #[serde(rename = "@I")]
        intensity: Color,
        #[serde(rename = "@from")]
        from: Vec3DTO,
        #[serde(rename = "@to")]
        to: Vec3DTO,
        #[serde(rename = "@cutoff", deserialize_with = "parse_from_string")]
        cutoff: f32,
        #[serde(rename = "@falloff", deserialize_with = "parse_from_string")]
        falloff: f32,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum ObjectDTO {
    Sphere {
        #[serde(rename = "@center")]
        center: Vec3DTO,
        #[serde(rename = "@radius", deserialize_with = "parse_from_string")]
        radius: f32,
    },
    Plane {
        #[serde(rename = "@point")]
        point: Vec3DTO,
        #[serde(rename = "@normal")]
        normal: Vec3DTO,
    },
    #[serde(rename = "trianglemesh")]
    TriangleMesh(TriangleMeshDTO),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TriangleMeshDTO {
    File {
        #[serde(rename = "@filename")]
        filename: String,
    },
    Inline {
        #[serde(rename = "@ntriangles", deserialize_with = "parse_from_string")]
        ntriangles: usize,
        #[serde(rename = "@vertices")]
        vertices: Vec3Array,
        #[serde(rename = "@vertex_indices")]
        vertex_indices: ArrayDTO<u32>,
        #[serde(rename = "@normals")]
        normals: Vec3Array,
        #[serde(rename = "@normal_indices")]
        normal_indices: ArrayDTO<u32>,
        #[serde(rename = "@uvs")]
        uvs: Option<Vec2Array>,
        #[serde(rename = "@uv_indices")]
        uv_indices: Option<ArrayDTO<u32>>,
        #[serde(
            rename = "@reverse_vertex_order",
            deserialize_with = "parse_from_string",
            default
        )]
        reverse_vertex_order: bool,
        #[serde(
            rename = "@compute_normals",
            deserialize_with = "parse_optional_from_string",
            default
        )]
        compute_normals: Option<bool>,
        #[serde(
            rename = "@backface_cull",
            deserialize_with = "parse_from_string",
            default
        )]
        backface_cull: bool,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum MaterialDTO {
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
    Cel {
        #[serde(rename = "@color_map")]
        color_map: Colors,
        #[serde(rename = "@ambient", default)]
        ambient: Color,
        #[serde(rename = "@shadow_color")]
        shadow_color: Option<ColorU8>,
        #[serde(
            rename = "@silhouette_angle",
            deserialize_with = "parse_optional_from_string",
            default
        )]
        silhouette_angle: Option<f32>,
        #[serde(rename = "@silhouette_color", default)]
        silhouette_color: Color,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum IntegratorDTO {
    Flat,
    NormalMap,
    BlinnPhong {
        #[serde(rename = "@depth", deserialize_with = "parse_from_string")]
        depth: u8,
    },
    CelShading {
        #[serde(rename = "@mapping_interval")]
        mapping_interval: ArrayDTO<u8>,
    },
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "@type")]
#[serde(rename_all = "snake_case")]
pub enum AggregatorDTO {
    List,
    Tree,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3DTO {
    x: f32,
    y: f32,
    z: f32,
}

impl<'de> Deserialize<'de> for Vec3DTO {
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

        Ok(Vec3DTO { x, y, z })
    }
}

impl From<Vec3DTO> for Vec3A {
    fn from(value: Vec3DTO) -> Self {
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

            colors.push(ColorU8 { red, green, blue }.into());
        }

        Ok(Self(colors))
    }
}

#[derive(Debug, Clone)]
pub struct Vec3Array(pub Vec<Vec3A>);

impl<'de> Deserialize<'de> for Vec3Array {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() % 3 != 0 {
            return Err(serde::de::Error::custom(
                "Expected a multiple of 3 coordinate components",
            ));
        }

        let mut vecs = Vec::new();

        for vec in parts.chunks_exact(3) {
            let x = vec[0].parse().map_err(serde::de::Error::custom)?;
            let y = vec[1].parse().map_err(serde::de::Error::custom)?;
            let z = vec[2].parse().map_err(serde::de::Error::custom)?;

            vecs.push(Vec3A::new(x, y, z));
        }

        Ok(Self(vecs))
    }
}

#[derive(Debug, Clone)]
pub struct Vec2Array(pub Vec<Vec2>);

impl<'de> Deserialize<'de> for Vec2Array {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() % 2 != 0 {
            return Err(serde::de::Error::custom(
                "Expected a multiple of 2 coordinate components",
            ));
        }

        let mut vecs = Vec::new();

        for vec in parts.chunks_exact(3) {
            let x = vec[0].parse().map_err(serde::de::Error::custom)?;
            let y = vec[1].parse().map_err(serde::de::Error::custom)?;

            vecs.push(Vec2::new(x, y));
        }

        Ok(Self(vecs))
    }
}

#[derive(Debug, Clone)]
pub struct ArrayDTO<T>(pub Vec<T>);

impl<'de, T> Deserialize<'de> for ArrayDTO<T>
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
