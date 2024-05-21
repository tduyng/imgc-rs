mod cli;
mod converter;

use clap::Parser;
use cli::CliArgs;
use converter::convert_images;
use std::path::Path;

/// Entry point of the `imgc` application.
///
/// Parses command-line arguments and initiates the image conversion process.
/// If an error occurs during conversion, it is printed to the standard error output.
fn main() {
    let args = CliArgs::parse();
    let dir = args.dir;
    let output_dir = args.output;

    let path = Path::new(&dir);
    if let Err(e) = convert_images(path, &output_dir) {
        eprint!("Error {}", e)
    }
}
