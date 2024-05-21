use clap::Parser;
use imgc::{
    cli::{CliArgs, Command},
    converter::convert_images,
    format::ImageFormat,
    utils::remove_files,
};
use std::path::Path;

fn main() {
    let args = CliArgs::parse();
    match args.command {
        Command::Webp { dir, output } => {
            if let Err(e) = convert_images(Path::new(&dir), &output, &ImageFormat::Webp) {
                eprintln!("Error: {}", e);
            }
        }
        Command::Clean { dir, ext } => {
            let dir_path = Path::new(&dir);
            if let Err(e) = remove_files(dir_path, &ext) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
