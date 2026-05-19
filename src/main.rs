use anyhow::Result;
use clap::Parser;
use raytracer::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();

    raytracer::run(&args)?;

    Ok(())
}
