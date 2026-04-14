use std::{
    fs::File,
    io::{BufWriter, Write},
};

use serde::Deserialize;

use crate::Result;
use crate::{Point2, RGBColor};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ImageType {
    Ppm,
    Png,
}

pub(crate) struct Film {
    width: u16,
    height: u16,
    filename: String,
    img_type: ImageType,
    buffer: Vec<RGBColor>,
}

impl Film {
    const MAX_CHANNEL_VALUE: u8 = u8::MAX;

    pub fn new(width: u16, height: u16, filename: &str, img_type: ImageType) -> Self {
        let buffer = vec![RGBColor::default(); width as usize * height as usize];

        Self {
            width,
            height,
            filename: filename.into(),
            img_type,
            buffer,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn add_sample(&mut self, point: Point2, color: RGBColor) {
        let index = point.row as usize * self.width as usize + point.col as usize;
        self.buffer[index] = color;
    }

    pub fn write_image(&self) -> Result<()> {
        let file = File::create(&self.filename)?;
        let w = BufWriter::new(file);

        match self.img_type {
            ImageType::Ppm => self.write_ppm(w),
            ImageType::Png => self.write_png(w),
        }
    }

    fn write_ppm(&self, mut out: BufWriter<File>) -> Result<()> {
        writeln!(out, "P3")?;
        writeln!(out, "{} {}", self.width, self.height)?;
        writeln!(out, "{}", Self::MAX_CHANNEL_VALUE)?;

        for color in &self.buffer {
            writeln!(out, "{} {} {}", color.red, color.green, color.blue)?;
        }

        Ok(())
    }

    fn write_png(&self, out: BufWriter<File>) -> Result<()> {
        let mut encoder = png::Encoder::new(out, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(bytemuck::cast_slice(&self.buffer))?;

        Ok(())
    }
}
