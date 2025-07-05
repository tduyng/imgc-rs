/// This module provides functionality for converting images to different formats.
pub mod webp;

use crate::{converter::webp::encode_webp, format::ImageFormat, utils::is_supported, Error};
use image::ImageReader;
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Processes and encodes images in a given directory to the specified image format.
pub fn convert_images(
    pattern: &str,
    output: &Option<String>,
    img_format: &ImageFormat,
) -> Result<(), Error> {
    let paths: Vec<PathBuf> = glob::glob(pattern)?
        .filter_map(|entry| entry.ok())
        .collect();

    paths
        .par_iter()
        .filter(|path| is_supported(path, img_format))
        .try_for_each(|path| convert_image(path, output, img_format))?;

    Ok(())
}

/// Encodes an image to the specified image format and saves it to the specified output directory.
fn convert_image(
    input_path: &Path,
    output_dir: &Option<String>,
    img_format: &ImageFormat,
) -> Result<(), Error> {
    let image_reader = ImageReader::open(input_path)?;
    let image = image_reader.decode()?;

    let image_data = match img_format {
        ImageFormat::Webp => encode_webp(&image)?,
        _ => return Err(Error::from_string("Unsupported image format".to_string())),
    };

    let ext = img_format.extension();
    let output_path = if let Some(output_dir) = output_dir {
        Path::new(&output_dir)
            .join(input_path.file_stem().unwrap())
            .with_extension(ext)
    } else {
        input_path.with_extension(ext)
    };

    fs::write(output_path.clone(), image_data)?;
    println!("Generated: {}", output_path.display());

    Ok(())
}
