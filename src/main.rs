use clap::Parser;
use imgc::{
    cli::{CliArgs, Command},
    converter::convert_images,
    format::ImageFormat,
    utils::remove_files,
    Error,
};

fn main() -> Result<(), Error> {
    let args = CliArgs::parse();
    match args.command {
        Command::Webp { pattern, output } => convert_images(&pattern, &output, &ImageFormat::Webp)?,
        Command::Clean { pattern } => remove_files(&pattern)?,
    }
    Ok(())
}
