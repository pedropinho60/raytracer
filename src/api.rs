use crate::{
    RGBColor,
    camera::{Camera, OrthographicCamera, PerspectiveCamera},
    cli::Cli,
    error::SceneError,
    math::{Point2, Point3, Vec3},
    object::{Object, Sphere},
    parse::{CameraType, ObjectType},
    scene::Scene,
};
use std::fs;

use quick_xml::de::from_str;

use crate::{
    Result,
    background::{Background, GradientBackground, SingleColorBackground},
    film::Film,
    parse::{BackgroundType, FilmType, Rt3, SceneCommand},
};

pub struct Api {
    scene: Scene,
}

impl Api {
    pub fn init(args: Cli) -> Result<Self, SceneError> {
        let xml_data = fs::read_to_string(args.input_scene_file)?;

        let scene = from_str::<Rt3>(&xml_data)?;

        let mut parsed_film = None;
        let mut parsed_background: Option<Box<dyn Background>> = None;
        let mut parsed_camera_args: Option<(Point3, Point3, Vec3)> = None;
        let mut parsed_camera_type = None;

        let mut objects: Vec<Box<dyn Object>> = Vec::new();

        for command in scene.commands {
            match command {
                SceneCommand::Film(FilmType::Image {
                    w_res,
                    h_res,
                    filename,
                    img_type,
                }) => parsed_film = Some(Film::new(w_res, h_res, filename, img_type)),
                SceneCommand::Background(background_type) => match background_type {
                    BackgroundType::SingleColor { color } => {
                        parsed_background = Some(Box::new(SingleColorBackground::new(color)))
                    }
                    BackgroundType::FourColors { bl, tl, tr, br } => {
                        parsed_background = Some(Box::new(GradientBackground::new(tl, tr, bl, br)))
                    }
                },
                SceneCommand::Camera(camera_type) => {
                    parsed_camera_type = Some(camera_type);
                }
                SceneCommand::Lookat {
                    look_from,
                    look_at,
                    up,
                } => parsed_camera_args = Some((look_at, look_from, up)),
                SceneCommand::Object(object_type) => match object_type {
                    ObjectType::Sphere { center, radius } => {
                        objects.push(Box::new(Sphere { center, radius }))
                    }
                },
                _ => (),
            }
        }

        let background = parsed_background.ok_or(SceneError::MissingComponent(
            "missing background definition",
        ))?;

        let film = parsed_film.ok_or(SceneError::MissingComponent("missing film definition"))?;

        let (look_at, look_from, up) =
            parsed_camera_args.ok_or(SceneError::MissingComponent("missing lookat definition"))?;

        let camera_type =
            parsed_camera_type.ok_or(SceneError::MissingComponent("missing camera definition"))?;

        let camera: Box<dyn Camera> = match camera_type {
            CameraType::Orthographic { screen_window } => Box::new(OrthographicCamera::new(
                look_at,
                look_from,
                up,
                screen_window,
                film,
            )),
            CameraType::Perspective { fovy } => {
                Box::new(PerspectiveCamera::new(look_at, look_from, up, fovy, film))
            }
        };

        Ok(Self {
            scene: Scene {
                background,
                camera,
                objects,
            },
        })
    }

    pub fn render(&mut self) -> Result<()> {
        let camera = &mut self.scene.camera;
        let background = &self.scene.background;

        let height = camera.film().height();
        let width = camera.film().width();

        for row in 0..height {
            let normalized_row = row as f64 / (height - 1) as f64;

            for col in 0..width {
                let normalized_col = col as f64 / (width - 1) as f64;

                let mut color = background.sample(normalized_row, normalized_col);

                let ray = camera.generate_ray(Point2 { row, col });

                for object in &self.scene.objects {
                    if object.intersect_p(ray) {
                        color = RGBColor {
                            red: 255,
                            green: 0,
                            blue: 0,
                        }
                    }
                }

                camera.film().add_sample(Point2 { row, col }, color);
            }
        }

        camera.film().write_image()?;

        Ok(())
    }
}
