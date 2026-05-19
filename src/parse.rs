pub mod dto;

use std::path::PathBuf;

use serde::Deserialize;

use crate::parse::dto::{
    AggregatorDTO, BackgroundDTO, CameraArgsDTO, CameraDTO, FilmDTO, IntegratorDTO, LightDTO,
    MaterialDTO, ObjectDTO,
};

#[derive(Debug, Deserialize)]
#[serde(rename = "RT3")]
pub struct Rt3 {
    #[serde(rename = "$value")]
    pub commands: Vec<SceneCommand>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneCommand {
    Lookat(CameraArgsDTO),
    Camera(CameraDTO),
    Integrator(IntegratorDTO),
    Film(FilmDTO),
    Aggregator(AggregatorDTO),
    WorldBegin,
    MakeNamedMaterial {
        #[serde(rename = "@name")]
        name: String,
        #[serde(flatten)]
        material_type: MaterialDTO,
    },
    NamedMaterial {
        #[serde(rename = "@name")]
        name: String,
    },
    Material(MaterialDTO),
    Object(ObjectDTO),
    Background(BackgroundDTO),
    LightSource(LightDTO),
    WorldEnd,
    RenderAgain,
    Include {
        #[serde(rename = "@filename")]
        filename: PathBuf,
    },
}
