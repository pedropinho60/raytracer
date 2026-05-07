use crate::{
    camera::Camera,
    cli::Cli,
    error::SceneError,
    light::Light,
    material::Material,
    parse::{CameraArgs, CameraType, IntegratorType},
    primitive::AggregatePrimitive,
    scene::Scene,
};
use std::{collections::HashMap, fs, path::Path};

use quick_xml::de::from_str;

use crate::{
    Result,
    background::Background,
    film::Film,
    parse::{FilmType, Rt3, SceneCommand},
};

pub struct RenderState {
    pub current_film: Option<Film>,
    pub current_background: Option<Background>,
    pub current_camera_args: Option<CameraArgs>,
    pub current_camera_type: Option<CameraType>,
    pub current_integrator_type: Option<IntegratorType>,

    pub materials: Vec<Material>,
    pub material_names: HashMap<String, usize>,
    pub primitives: AggregatePrimitive,
    pub lights: Vec<Light>,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            current_film: None,
            current_background: None,
            current_camera_args: None,
            current_camera_type: None,
            current_integrator_type: None,
            materials: Vec::new(),
            material_names: HashMap::new(),
            primitives: AggregatePrimitive::new(),
            lights: Vec::new(),
        }
    }

    pub fn execute_render(&mut self) -> Result<()> {
        let film = self
            .current_film
            .as_mut()
            .ok_or(SceneError::Render("cannot render without a film"))?;

        let background = self
            .current_background
            .as_ref()
            .ok_or(SceneError::Render("cannot render without a background"))?;

        let camera_args = self
            .current_camera_args
            .ok_or(SceneError::Render("cannot render without lookat"))?;

        let camera_type = self
            .current_camera_type
            .ok_or(SceneError::Render("cannot render without a camera"))?;

        let mut camera: Camera = camera_type.to_camera(camera_args, film.width(), film.height());

        let integrator_type = self
            .current_integrator_type
            .ok_or(SceneError::Render("cannot render without an integrator"))?;

        let scene = Scene {
            background,
            materials: &self.materials,
            primitives: &self.primitives,
            lights: &self.lights,
        };

        let mut integrator = integrator_type.to_integrator();

        integrator.render(&mut camera, &scene, film)?;

        Ok(())
    }
}

impl Default for RenderState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run(args: Cli) -> Result<()> {
    let mut state = RenderState::new();
    parse_from_file(&args.input_scene_file, &mut state)?;

    Ok(())
}

fn parse_from_file(file_path: &Path, state: &mut RenderState) -> Result<()> {
    let xml_data = fs::read_to_string(file_path)?;

    let scene = from_str::<Rt3>(&xml_data)?;

    let mut current_material = None;

    for command in scene.commands {
        match command {
            SceneCommand::Film(FilmType::Image {
                width,
                height,
                filename,
                img_type,
                gamma_corrected,
                dithering,
            }) => {
                state.current_film = Some(Film::new(
                    width,
                    height,
                    filename,
                    img_type,
                    gamma_corrected,
                    dithering.to_dithering(),
                ))
            }
            SceneCommand::Background(background_type) => {
                let background = background_type.to_background();

                state.current_background = Some(background);
            }
            SceneCommand::Camera(camera_type) => {
                state.current_camera_type = Some(camera_type);
            }
            SceneCommand::Lookat(camera_args) => state.current_camera_args = Some(camera_args),
            SceneCommand::Object(object_type) => {
                let material_id = current_material
                    .ok_or(SceneError::MissingComponent("missing material for object"))?;

                state.primitives.add(object_type.to_primitive(material_id))
            }
            SceneCommand::Material(material_type) => {
                let material = material_type.to_material();

                let index = state.materials.len();
                state.materials.push(material);

                current_material = Some(index);
            }
            SceneCommand::MakeNamedMaterial {
                name,
                material_type,
            } => {
                let material = material_type.to_material();

                let index = state.materials.len();
                state.materials.push(material);

                state.material_names.insert(name, index);
            }
            SceneCommand::NamedMaterial { name } => {
                current_material = Some(*state.material_names.get(&name).ok_or(
                    SceneError::MissingComponent("material `{name}` does not exist"),
                )?);
            }
            SceneCommand::Integrator(integrator_type) => {
                state.current_integrator_type = Some(integrator_type)
            }
            SceneCommand::WorldBegin => (),
            SceneCommand::WorldEnd => state.execute_render()?,
            SceneCommand::RenderAgain => state.execute_render()?,
            SceneCommand::Include { filename } => {
                let current_dir = file_path.parent().unwrap_or_else(|| Path::new(""));

                let resolved_path = current_dir.join(filename);

                parse_from_file(&resolved_path, state)?;
            }
            SceneCommand::LightSource(light_type) => state.lights.push(light_type.to_light()),
            SceneCommand::Aggregator { ty: _ty } => (),
        }
    }

    Ok(())
}
