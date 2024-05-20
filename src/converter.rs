use image::io::Reader;
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};
use crate::webp::to_webp;

pub fn convert_images(path: &Path, output_dir: &Option<String>) -> Result<(), String> {
    if path.is_dir() {
        let entries: Vec<PathBuf> = fs::read_dir(path)
            .map_err(|e| format!("Read directory failed: {e}"))?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();

        entries.par_iter().try_for_each(|path| {
            if path.is_dir() {
                convert_images(path, output_dir)
            } else if is_supported_image(path) {
                convert_to_webp(path, output_dir)
            } else {
                Ok(())
            }
        })?;
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
        }
        Err(_) => false,
    }
}

fn convert_to_webp(input_path: &Path, output_dir: &Option<String>) -> Result<(), String> {
    let image_render =
        Reader::open(input_path).map_err(|e| format!("Failed to open image: {}", e))?;
    let image = image_render
        .decode()
        .map_err(|e| format!("Failed to decode image: {}\n", e))?;
    let webp_data = to_webp(&image)?;

    let output_path = if let Some(output_dir) = output_dir {
        Path::new(output_dir)
            .join(input_path.file_stem().unwrap())
            .with_extension("webp")
    } else {
        input_path.with_extension("webp")
    };

    fs::write(output_path.clone(), webp_data)
        .map_err(|e| format!("Failed to write WebP file: {}", e))?;

    println!("Generated: {}", output_path.display());

    Ok(())
}
