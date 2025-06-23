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
        Command::Webp { pattern, output, overwrite_existing, lossless, quality } => convert_images(&pattern, &ImageFormat::Webp, &output, &overwrite_existing, &lossless, &quality, &None)?,
        Command::Avif { pattern, output, overwrite_existing, quality, speed} => convert_images(&pattern, &ImageFormat::Avif, &output, &overwrite_existing, &None, &quality, &speed)?,
        Command::Clean { pattern } => remove_files(&pattern)?,
    }
    Ok(())
}
