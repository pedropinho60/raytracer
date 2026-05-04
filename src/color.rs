use std::ops::{Add, AddAssign, Mul};

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Default)]
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

    pub fn luminosity(self) -> f32 {
        0.2126 * self.red + 0.7152 * self.green + 0.0722 * self.blue
    }

    pub fn clamp(self, min: f32, max: f32) -> Color {
        Self {
            red: self.red.clamp(min, max),
            green: self.green.clamp(min, max),
            blue: self.blue.clamp(min, max),
        }
    }

    pub fn gamma_corrected(self) -> Color {
        let gamma = 1.0 / 2.2;

        Color {
            red: self.red.powf(gamma),
            green: self.green.powf(gamma),
            blue: self.blue.powf(gamma),
        }
    }
}

impl From<ColorU8> for Color {
    fn from(value: ColorU8) -> Self {
        Self {
            red: value.red as f32 / 255.0,
            green: value.green as f32 / 255.0,
            blue: value.blue as f32 / 255.0,
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
