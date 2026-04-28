use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use serde::Deserialize;

use crate::Result;
use crate::color::Color;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageType {
    Ppm,
    Png,
}

#[derive(Debug, Clone)]
pub struct Film {
    width: u16,
    height: u16,
    filename: PathBuf,
    img_type: ImageType,
    buffer: Vec<Color>,
    gamma_corrected: bool,
}

impl Film {
    pub fn new(
        width: u16,
        height: u16,
        filename: PathBuf,
        img_type: ImageType,
        gamma_corrected: bool,
    ) -> Self {
        let buffer = vec![Color::default(); width as usize * height as usize];

        Self {
            width,
            height,
            filename,
            img_type,
            buffer,
            gamma_corrected,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn pixels_mut(&mut self) -> &mut [Color] {
        &mut self.buffer
    }

    pub fn write_image(&self) -> Result<()> {
        let file = File::create(&self.filename)?;
        let w = BufWriter::new(file);

        let mut final_pixels = Vec::with_capacity(self.buffer.len() * 3);

        for &spectrum in &self.buffer {
            let r = spectrum.red.clamp(0.0, 1.0);
            let g = spectrum.green.clamp(0.0, 1.0);
            let b = spectrum.blue.clamp(0.0, 1.0);

            let (final_r, final_g, final_b) = if self.gamma_corrected {
                let gamma = 1.0 / 2.2;
                (r.powf(gamma), g.powf(gamma), b.powf(gamma))
            } else {
                (r, g, b)
            };

            final_pixels.push((final_r * 255.0) as u8);
            final_pixels.push((final_g * 255.0) as u8);
            final_pixels.push((final_b * 255.0) as u8);
        }

        match self.img_type {
            ImageType::Ppm => self.write_ppm(w, &final_pixels),
            ImageType::Png => self.write_png(w, &final_pixels),
        }
    }

    fn write_ppm(&self, mut out: BufWriter<File>, buffer: &[u8]) -> Result<()> {
        writeln!(out, "P3")?;
        writeln!(out, "{} {}", self.width, self.height)?;
        writeln!(out, "{}", 255.0)?;

        for color in buffer {
            writeln!(out, "{}", color)?;
        }

        Ok(())
    }

    fn write_png(&self, out: BufWriter<File>, buffer: &[u8]) -> Result<()> {
        let mut encoder = png::Encoder::new(out, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(buffer)?;

        Ok(())
    }
}
