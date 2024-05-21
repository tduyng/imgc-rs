mod cli;
mod converter;

use clap::Parser;
use cli::CliArgs;
use converter::convert_images;
use std::path::Path;

fn main() {
    let args = CliArgs::parse();
    let dir = args.dir;
    let output_dir = args.output;

    let path = Path::new(&dir);
    if let Err(e) = convert_images(path, &output_dir) {
        eprint!("Error {}", e)
    }
}
