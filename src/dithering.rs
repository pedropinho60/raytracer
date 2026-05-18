use std::io::Cursor;

use derive_more::From;

use crate::color::Color;

#[derive(Debug, Clone, From)]
pub enum Dithering {
    None,
    Bayer(BayerDithering),
    WhiteNoise(WhiteNoiseDithering),
    BlueNoise(BlueNoiseDithering),
}

impl Dithering {
    pub fn get_color(&self, row: usize, col: usize, color: Color) -> Color {
        match self {
            Dithering::None => color,
            Dithering::Bayer(inner) => inner.get_color(row, col, color),
            Dithering::WhiteNoise(inner) => inner.get_color(color),
            Dithering::BlueNoise(inner) => inner.get_color(row, col, color),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BayerDithering;

impl BayerDithering {
    const MATRIX: [[f32; 4]; 4] = [
        [0.0, 8.0, 2.0, 10.0],
        [12.0, 4.0, 14.0, 6.0],
        [3.0, 11.0, 1.0, 9.0],
        [15.0, 7.0, 13.0, 5.0],
    ];

    pub fn get_color(&self, row: usize, col: usize, color: Color) -> Color {
        let luminance = color.luminance();
        let threshold = Self::MATRIX[row % 4][col % 4] / 16.0;

        if luminance > threshold {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhiteNoiseDithering;

impl WhiteNoiseDithering {
    pub fn get_color(&self, color: Color) -> Color {
        let luminance = color.luminance();
        let threshold = rand::random();

        if luminance > threshold {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlueNoiseDithering {
    blue_noise: Vec<f32>,
    width: usize,
    height: usize,
}

impl BlueNoiseDithering {
    pub fn new() -> Self {
        let bytes = include_bytes!("../assets/blue_noise.png");

        let decoder = png::Decoder::new(Cursor::new(bytes));
        let mut reader = decoder.read_info().unwrap();

        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();

        let bytes = &buf[..info.buffer_size()];
        let width = info.width as usize;
        let height = info.height as usize;

        let mut data = vec![0.0f32; width * height];

        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) * 4;
                let r = bytes[index];

                data[y * width + x] = r as f32 / 255.0;
            }
        }

        Self {
            blue_noise: data,
            width,
            height,
        }
    }

    pub fn get_color(&self, row: usize, col: usize, color: Color) -> Color {
        let row = row % self.height;
        let col = col % self.width;
        let threshold = self.blue_noise[row * self.width + col];

        if color.luminance() > threshold {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}
