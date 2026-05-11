use serde::Deserialize;

mod aggregator;
mod api;
mod background;
mod bounding_box;
mod camera;
mod cli;
mod color;
mod dithering;
mod error;
mod film;
mod hittable;
mod integrator;
mod light;
mod material;
mod parse;
mod primitive;
mod ray;
mod scene;
mod surfel;

pub use api::run;
pub use cli::Cli;
pub use error::Result;

#[derive(Debug, Clone, Copy)]
pub struct WindowSize {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl<'de> Deserialize<'de> for WindowSize {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() != 4 {
            return Err(serde::de::Error::custom(
                "Expected exactly 4 size components",
            ));
        }

        let left = parts[0].parse().map_err(serde::de::Error::custom)?;
        let right = parts[1].parse().map_err(serde::de::Error::custom)?;
        let bottom = parts[2].parse().map_err(serde::de::Error::custom)?;
        let top = parts[3].parse().map_err(serde::de::Error::custom)?;

        Ok(WindowSize {
            left,
            right,
            bottom,
            top,
        })
    }
}
