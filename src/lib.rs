use bytemuck::{Pod, Zeroable};
use serde::Deserialize;

mod api;
mod background;
mod cli;
mod error;
mod film;
mod parse;

pub use api::Api;
pub use cli::Cli;
pub use error::Result;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub(crate) struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl<'de> Deserialize<'de> for RGBColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(serde::de::Error::custom(
                "Expected exactly 3 color components",
            ));
        }

        let red = parts[0].parse().map_err(serde::de::Error::custom)?;
        let green = parts[1].parse().map_err(serde::de::Error::custom)?;
        let blue = parts[2].parse().map_err(serde::de::Error::custom)?;

        Ok(RGBColor { red, green, blue })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Point2 {
    pub row: u16,
    pub col: u16,
}
