use std::io;

use thiserror::Error;

pub type Result<T, E = SceneError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum SceneError {
    #[error("Failed to parse XML")]
    XmlParse(#[from] quick_xml::DeError),

    #[error("Failed to read file")]
    Io(#[from] io::Error),

    #[error("Invalid scene definition")]
    MissingComponent(&'static str),

    #[error("Failed to encode PNG")]
    PngEncoding(#[from] png::EncodingError),
}
