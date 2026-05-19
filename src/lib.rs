#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::many_single_char_names)]

mod api;
mod cli;
mod core;
mod error;

mod geometry;
mod parse;
mod render;
mod scene;

pub use api::run;
pub use cli::Cli;
pub use error::Result;
