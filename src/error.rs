use std::io;

use thiserror::Error;

pub type Result<T, E = SceneError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum SceneError {
    #[error("Failed to parse XML: \n{0}")]
    XmlParse(String),

    #[error("Failed to read file")]
    Io(#[from] io::Error),

    #[error("Invalid scene definition: {0}")]
    MissingComponent(String),

    #[error("Error while rendering: {0}")]
    Render(String),

    #[error("Failed to encode PNG")]
    PngEncoding(#[from] png::EncodingError),
}
