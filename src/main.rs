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
        Command::Webp { pattern, output, lossless, quality } => convert_images(&pattern, &ImageFormat::Webp, &output, &lossless, &quality, &None)?,
        Command::Avif { pattern, output, quality, speed} => convert_images(&pattern, &ImageFormat::Avif, &output, &None, &quality, &speed)?,
        Command::Clean { pattern } => remove_files(&pattern)?,
    }
    Ok(())
}
