use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub input_scene_file: PathBuf,
}
