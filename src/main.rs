use anyhow::Result;
use clap::Parser;
use raytracer::{Api, Cli};

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut api = Api::init(args)?;

    api.render()?;

    Ok(())
}
