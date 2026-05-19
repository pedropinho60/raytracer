use std::{
    ops::{Add, AddAssign, Mul},
    sync::OnceLock,
};

use serde::Deserialize;

static GAMMA_LUT: OnceLock<[u8; 4096]> = OnceLock::new();

pub fn get_gamma_lut() -> &'static [u8; 4096] {
    GAMMA_LUT.get_or_init(|| {
        std::array::from_fn(|i| {
            let val = (i as f32) / 4095.0;
            let gamma_corrected = val.powf(1.0 / 2.2);

            (gamma_corrected * 255.99) as u8
        })
    })
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };

    pub fn lerp(a: Color, b: Color, t: f32) -> Self {
        Self {
            red: (1. - t) * a.red + t * b.red,
            green: (1. - t) * a.green + t * b.green,
            blue: (1. - t) * a.blue + t * b.blue,
        }
    }

    pub fn luminance(self) -> f32 {
        0.2126 * self.red + 0.7152 * self.green + 0.0722 * self.blue
    }

    pub fn clamp(self, min: f32, max: f32) -> Color {
        Self {
            red: self.red.max(min).min(max),
            green: self.green.max(min).min(max),
            blue: self.blue.max(min).min(max),
        }
    }
}

impl From<ColorU8> for Color {
    fn from(value: ColorU8) -> Self {
        Self {
            red: f32::from(value.red) / 255.0,
            green: f32::from(value.green) / 255.0,
            blue: f32::from(value.blue) / 255.0,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl AddAssign<&Color> for Color {
    fn add_assign(&mut self, rhs: &Color) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::Output {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl<'de> Deserialize<'de> for Color {
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

        Ok(Color { red, green, blue })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorU8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<Color> for ColorU8 {
    fn from(value: Color) -> Self {
        Self {
            red: (value.red * 255.99) as u8,
            green: (value.green * 255.99) as u8,
            blue: (value.blue * 255.99) as u8,
        }
    }
}

impl<'de> Deserialize<'de> for ColorU8 {
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

        Ok(ColorU8 { red, green, blue })
    }
}
