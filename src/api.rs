use std::{collections::HashMap, fs, path::Path, time::Instant};

use crate::{
    Result,
    cli::Cli,
    error::SceneError,
    geometry::hittable::Hittable,
    parse::{
        Rt3, SceneCommand,
        dto::{AggregatorDTO, CameraArgsDTO, CameraDTO, FilmDTO, IntegratorDTO},
    },
    render::{aggregator::PrimitiveAggregator, film::Film, integrator::Integrator},
    scene::{Scene, background::Background, camera::Camera, light::Light, material::Material},
};

pub struct RenderState {
    pub current_film: Option<Film>,
    pub current_background: Option<Background>,
    pub current_camera_args: Option<CameraArgsDTO>,
    pub current_camera_dto: Option<CameraDTO>,
    pub current_integrator_dto: Option<IntegratorDTO>,
    pub current_aggregator_dto: Option<AggregatorDTO>,

    pub materials: Vec<Material>,
    pub material_names: HashMap<String, usize>,
    pub primitives: Vec<Hittable>,
    pub lights: Vec<Light>,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            current_film: None,
            current_background: None,
            current_camera_args: None,
            current_camera_dto: None,
            current_integrator_dto: None,
            current_aggregator_dto: None,
            materials: Vec::new(),
            material_names: HashMap::new(),
            primitives: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn execute_render(&mut self) -> Result<()> {
        let film = self
            .current_film
            .as_mut()
            .ok_or(SceneError::Render("cannot render without a film".into()))?;

        let background = self.current_background.as_ref().ok_or(SceneError::Render(
            "cannot render without a background".into(),
        ))?;

        let camera_args = self
            .current_camera_args
            .ok_or(SceneError::Render("cannot render without lookat".into()))?;

        let camera_dto = self
            .current_camera_dto
            .ok_or(SceneError::Render("cannot render without a camera".into()))?;

        let mut camera = Camera::build(camera_dto, camera_args, film.width(), film.height());

        let integrator_dto = self
            .current_integrator_dto
            .as_ref()
            .ok_or(SceneError::Render(
                "cannot render without an integrator".into(),
            ))?;

        let aggregator_dto = self.current_aggregator_dto.ok_or(SceneError::Render(
            "cannot render without an object aggregator".into(),
        ))?;

        let start = Instant::now();

        let aggregator = PrimitiveAggregator::build(aggregator_dto, self.primitives.clone());

        let scene = Scene {
            background,
            materials: &self.materials,
            primitives: &aggregator,
            lights: &self.lights,
        };

        let mut integrator: Integrator = integrator_dto.into();

        let render_start = Instant::now();
        integrator.render(&mut camera, &scene, film);
        let image_elapsed = render_start.elapsed();

        println!("Render time: {image_elapsed:?}");

        let image_start = Instant::now();
        film.write_image()?;
        let image_elapsed = image_start.elapsed();
        println!("Time to write image: {image_elapsed:?}");

        let total = start.elapsed();
        println!("Total time: {total:?}");

        Ok(())
    }
}

impl Default for RenderState {
    fn default() -> Self {
        Self::new()
    }
}

fn load_rt3_file(file_path: &Path) -> Result<Rt3> {
    let xml_data = fs::read_to_string(file_path)?;

    let mut deserializer = quick_xml::de::Deserializer::from_str(&xml_data);

    match serde_path_to_error::deserialize::<_, Rt3>(&mut deserializer) {
        Ok(scene) => Ok(scene),
        Err(err) => {
            let path = err.path().to_string();
            let msg = err.into_inner().to_string();

            Err(SceneError::XmlParse(format!(
                "Parse error at `{path}`: {msg}"
            )))
        }
    }
}

pub struct SceneBuilder {
    pub state: RenderState,
    current_material: Option<usize>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            state: RenderState::new(),
            current_material: None,
        }
    }

    pub fn process_file<F>(&mut self, file_path: &Path, on_render: &mut F) -> Result<()>
    where
        F: FnMut(&mut RenderState) -> Result<()>,
    {
        let rt3 = load_rt3_file(file_path)?;

        for command in rt3.commands {
            match command {
                SceneCommand::Include { filename } => {
                    let current_dir = file_path.parent().unwrap_or_else(|| Path::new(""));
                    let resolved_path = current_dir.join(filename);

                    self.process_file(&resolved_path, on_render)?;
                }
                SceneCommand::WorldBegin => (),
                SceneCommand::WorldEnd | SceneCommand::RenderAgain => on_render(&mut self.state)?,
                SceneCommand::Film(FilmDTO::Image {
                    width,
                    height,
                    filename,
                    img_type,
                    gamma_corrected,
                    dithering,
                }) => {
                    self.state.current_film = Some(Film::new(
                        width,
                        height,
                        filename,
                        img_type,
                        gamma_corrected,
                        dithering.into(),
                    ));
                }
                SceneCommand::Background(background_dto) => {
                    self.state.current_background = Some(background_dto.into());
                }
                SceneCommand::Camera(camera_dto) => {
                    self.state.current_camera_dto = Some(camera_dto);
                }
                SceneCommand::Lookat(camera_args) => {
                    self.state.current_camera_args = Some(camera_args);
                }
                SceneCommand::Object(object_dto) => {
                    let material_id = self.current_material.ok_or(SceneError::MissingComponent(
                        "missing material for object".into(),
                    ))?;

                    let primitives = Hittable::from_object(object_dto, material_id, file_path)?;

                    self.state.primitives.extend(primitives);
                }
                SceneCommand::Material(material_dto) => {
                    let material = material_dto.into();

                    let index = self.state.materials.len();
                    self.state.materials.push(material);

                    self.current_material = Some(index);
                }
                SceneCommand::MakeNamedMaterial {
                    name,
                    material_type: material_dto,
                } => {
                    let material = material_dto.into();

                    let index = self.state.materials.len();
                    self.state.materials.push(material);

                    self.state.material_names.insert(name, index);
                }
                SceneCommand::NamedMaterial { name } => {
                    self.current_material = Some(*self.state.material_names.get(&name).ok_or(
                        SceneError::MissingComponent(format!("material `{name}` does not exist")),
                    )?);
                }
                SceneCommand::Integrator(integrator_dto) => {
                    self.state.current_integrator_dto = Some(integrator_dto);
                }
                SceneCommand::LightSource(light_dto) => self.state.lights.push(light_dto.into()),
                SceneCommand::Aggregator(aggregator_dto) => {
                    self.state.current_aggregator_dto = Some(aggregator_dto);
                }
            }
        }

        Ok(())
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn run(args: &Cli) -> Result<()> {
    let mut builder = SceneBuilder::new();

    builder.process_file(&args.input_scene_file, &mut |state: &mut RenderState| {
        state.execute_render()?;

        Ok(())
    })?;

    Ok(())
}
