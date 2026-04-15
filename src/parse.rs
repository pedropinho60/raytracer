use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Deserializer};

use crate::{
    RGBColor, WindowSize,
    film::ImageType,
    math::{Point3, Vec3},
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
    Lookat {
        #[serde(rename = "@look_from")]
        look_from: Point3,
        #[serde(rename = "@look_at")]
        look_at: Point3,
        #[serde(rename = "@up")]
        up: Vec3,
    },
    Camera(CameraType),
    Integrator {
        #[serde(rename = "@type")]
        _integrator_type: String,
    },
    Film(FilmType),
    WorldBegin,
    Material(MaterialType),
    Object(ObjectType),
    Background(BackgroundType),
    WorldEnd,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
pub enum FilmType {
    #[serde(rename = "image")]
    Image {
        #[serde(rename = "@w_res", deserialize_with = "parse_number")]
        w_res: u16,
        #[serde(rename = "@h_res", deserialize_with = "parse_number")]
        h_res: u16,
        #[serde(rename = "@filename")]
        filename: PathBuf,
        #[serde(rename = "@img_type")]
        img_type: ImageType,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
pub enum BackgroundType {
    #[serde(rename = "single_color")]
    SingleColor {
        #[serde(rename = "@color")]
        color: RGBColor,
    },
    #[serde(rename = "4_colors")]
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

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
pub enum ObjectType {
    #[serde(rename = "sphere")]
    Sphere {
        #[serde(rename = "@center")]
        center: Point3,
        #[serde(rename = "@radius", deserialize_with = "parse_number")]
        radius: f64,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
pub enum MaterialType {
    #[serde(rename = "flat")]
    Flat {
        #[serde(rename = "@color")]
        color: String,
    },
}
