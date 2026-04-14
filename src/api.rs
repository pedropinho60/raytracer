use crate::{cli::Cli, error::SceneError};
use std::fs;

use quick_xml::de::from_str;

use crate::{
    Point2, Result,
    background::{Background, GradientBackground, SingleColorBackground},
    film::Film,
    parse::{BackgroundType, FilmType, Rt3, SceneCommand},
};

pub struct Api {
    background: Box<dyn Background>,
    film: Film,
}

impl Api {
    pub fn init(args: Cli) -> Result<Self, SceneError> {
        let xml_data = fs::read_to_string(args.input_scene_file)?;

        let scene = from_str::<Rt3>(&xml_data)?;

        let mut film = None;
        let mut background: Option<Box<dyn Background>> = None;

        for command in scene.commands {
            match command {
                SceneCommand::Film(FilmType::Image {
                    w_res,
                    h_res,
                    filename,
                    img_type,
                }) => film = Some(Film::new(w_res, h_res, &filename, img_type)),
                SceneCommand::Background(background_type) => match background_type {
                    BackgroundType::SingleColor { color } => {
                        background = Some(Box::new(SingleColorBackground::new(color)))
                    }
                    BackgroundType::FourColors { bl, tl, tr, br } => {
                        background = Some(Box::new(GradientBackground::new(tl, tr, bl, br)))
                    }
                },
                _ => (),
            }
        }

        Ok(Self {
            background: background.ok_or(SceneError::MissingComponent(
                "missing background definition",
            ))?,
            film: film.ok_or(SceneError::MissingComponent("missing film definition"))?,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        let height = self.film.height();
        let width = self.film.width();

        for row in 0..height {
            let normalized_row = row as f64 / (height - 1) as f64;

            for col in 0..width {
                let normalized_col = col as f64 / (width - 1) as f64;

                let color = self.background.sample(normalized_row, normalized_col);

                self.film.add_sample(Point2 { row, col }, color);
            }
        }

        self.film.write_image()?;

        Ok(())
    }
}
