use clap::Parser;
use image::{io::Reader, DynamicImage};
use std::{fs, path::Path};
use webp::{self, Encoder, WebPMemory};

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(short, long)]
    dir: Option<String>,

    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let args = CliArgs::parse();
    let dir = args.dir.unwrap_or_else(|| ".".to_string());
    let output_dir = args.output;

    let path = Path::new(&dir);
    match convert_images(path, &output_dir) {
        Ok(_) => println!("Print images succesfully"),
        Err(e) => eprint!("Error {}", e),
    }
}

fn convert_images(path: &Path, output_dir: &Option<String>) -> Result<(), String> {
    if path.is_dir() {
        let entries = fs::read_dir(path).map_err(|e| format!("Read directory failed: {e}"))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to load entry: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                convert_images(&path, output_dir)?;
            } else if is_supported_image(&path) {
                convert_to_webp(&path, output_dir)?;
            };
        }
    }
    Ok(())
}

fn is_supported_image(path: &Path) -> bool {
    match fs::read(path) {
        Ok(data) => {
            if path.extension().unwrap_or_default() == "webp" {
                return false;
            }
            image::guess_format(&data).is_ok()
        },
        Err(_) => false,
    }
}

pub fn to_webp(image: &DynamicImage) -> Result<WebPMemory, String> {
    let encoder = Encoder::from_image(image)
        .map_err(|e| format!("Failed to create a webp encoder: {}", e))?;
    let webp_data = encoder.encode(100.0);
    Ok(webp_data)
}

fn convert_to_webp(input_path: &Path, output_dir: &Option<String>) -> Result<(), String> {
    let image_render =
        Reader::open(input_path).map_err(|e| format!("Failed to open image: {}", e))?;
    let image = image_render
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;
    let webp_data = to_webp(&image)?;

    let output_path = if let Some(output_dir) = output_dir {
        Path::new(output_dir)
            .join(input_path.file_stem().unwrap())
            .with_extension("webp")
    } else {
        input_path.with_extension("webp")
    };

    fs::write(output_path.clone(), webp_data.to_vec())
        .map_err(|e| format!("Failed to write WebP file: {}", e))?;

    Ok(())
}
