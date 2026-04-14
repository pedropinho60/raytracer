use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::{RGBColor, film::ImageType};

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
pub(crate) struct Rt3 {
    #[serde(rename = "$value")]
    pub commands: Vec<SceneCommand>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SceneCommand {
    Camera(CameraType),
    Film(FilmType),
    WorldBegin,
    Background(BackgroundType),
    WorldEnd,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
pub enum CameraType {
    #[serde(rename = "orthographic")]
    Orthographic {
        #[serde(rename = "@screen_window")]
        screen_window: String,
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
        filename: String,
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
