use derive_more::From;
use serde::Deserialize;

use crate::{
    color::Color,
    math::{Point3, Vec3},
};

#[derive(Clone, From)]
pub enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
    Ambient(AmbientLight),
}

#[derive(Clone)]
pub struct PointLight {
    pub intensity: Color,
    pub point: Point3,
    pub attenuation: Attenuation,
}

impl PointLight {
    pub fn attenuation(&self, distance: f64) -> f64 {
        1.0 / (self.attenuation.kc
            + self.attenuation.kl * distance
            + self.attenuation.kq * distance)
    }
}

#[derive(Clone)]
pub struct DirectionalLight {
    pub intensity: Color,
    pub direction: Vec3,
}

#[derive(Clone)]
pub struct AmbientLight {
    pub intensity: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Attenuation {
    kc: f64,
    kl: f64,
    kq: f64,
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
