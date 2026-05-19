use derive_more::From;
use glam::Vec3A;
use serde::Deserialize;

use crate::{core::color::Color, parse::dto::LightDTO};

#[derive(Clone, From)]
pub enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
    Ambient(AmbientLight),
    Spotlight(Spotlight),
}

impl From<LightDTO> for Light {
    fn from(value: LightDTO) -> Self {
        match value {
            LightDTO::Ambient { intensity, scale } => AmbientLight {
                intensity: intensity * scale,
            }
            .into(),
            LightDTO::Point {
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
            LightDTO::Directional {
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
            LightDTO::Spotlight {
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

#[derive(Clone)]
pub struct PointLight {
    pub intensity: Color,
    pub point: Vec3A,
    pub attenuation: Attenuation,
}

impl PointLight {
    pub fn attenuation(&self, distance: f32) -> f32 {
        1.0 / (self.attenuation.kc
            + self.attenuation.kl * distance
            + self.attenuation.kq * distance)
    }
}

#[derive(Clone)]
pub struct DirectionalLight {
    pub intensity: Color,
    pub direction: Vec3A,
}

#[derive(Clone)]
pub struct AmbientLight {
    pub intensity: Color,
}

#[derive(Clone)]
pub struct Spotlight {
    pub intensity: Color,
    pub point: Vec3A,
    pub direction: Vec3A,
    pub cutoff_cos: f32,
    pub falloff_cos: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Attenuation {
    kc: f32,
    kl: f32,
    kq: f32,
}

impl Default for Attenuation {
    fn default() -> Self {
        Self {
            kc: 1.0,
            kl: 0.0,
            kq: 0.0,
        }
    }
}

impl<'de> Deserialize<'de> for Attenuation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(serde::de::Error::custom(
                "Expected exactly 3 attenuation components",
            ));
        }

        let kc = parts[0].parse().map_err(serde::de::Error::custom)?;
        let kl = parts[1].parse().map_err(serde::de::Error::custom)?;
        let kq = parts[2].parse().map_err(serde::de::Error::custom)?;

        Ok(Attenuation { kc, kl, kq })
    }
}
