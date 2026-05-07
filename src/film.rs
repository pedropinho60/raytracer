use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use rayon::prelude::*;
use serde::Deserialize;

use crate::color::{self, Color, ColorU8};
use crate::{Result, dithering::Dithering};

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
    dithering: Dithering,
}

impl Film {
    pub fn new(
        width: u16,
        height: u16,
        filename: PathBuf,
        img_type: ImageType,
        gamma_corrected: bool,
        dithering: Dithering,
    ) -> Self {
        let buffer = vec![Color::default(); width as usize * height as usize];

        Self {
            width,
            height,
            filename,
            img_type,
            buffer,
            gamma_corrected,
            dithering,
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

        let gamma_lut = color::get_gamma_lut();

        let width = self.width as usize;

        let final_pixels: Vec<u8> = self
            .buffer
            .par_iter()
            .enumerate()
            .flat_map_iter(|(i, color)| {
                let mut color_to_process = color.clamp(0.0, 1.0);

                let row = i / width;
                let col = i % width;

                color_to_process = self.dithering.get_color(row, col, color_to_process);

                let final_color = if self.gamma_corrected {
                    ColorU8 {
                        red: gamma_lut[(color_to_process.red * 4095.0) as usize],
                        green: gamma_lut[(color_to_process.green * 4095.0) as usize],
                        blue: gamma_lut[(color_to_process.blue * 4095.0) as usize],
                    }
                } else {
                    color_to_process.into()
                };

                [final_color.red, final_color.green, final_color.blue]
            })
            .collect();

        match self.img_type {
            ImageType::Ppm => self.write_ppm(w, &final_pixels),
            ImageType::Png => self.write_png(w, &final_pixels),
        }
    }

    fn write_ppm(&self, mut out: BufWriter<File>, buffer: &[u8]) -> Result<()> {
        writeln!(out, "P6")?;
        writeln!(out, "{} {}", self.width, self.height)?;
        writeln!(out, "{}", 255.0)?;

        out.write_all(buffer)?;

        Ok(())
    }

    fn write_png(&self, out: BufWriter<File>, buffer: &[u8]) -> Result<()> {
        let mut encoder = png::Encoder::new(out, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::NoCompression);
        encoder.set_filter(png::Filter::NoFilter);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(buffer)?;

        Ok(())
    }
}
