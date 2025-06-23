/// This module provides webp conversion
pub mod webp;
/// This module provides avif conversion
pub mod avif;

use crate::{converter::webp::encode_webp, converter::avif::encode_avif, format::ImageFormat, utils::is_supported, Error};
use image::ImageReader;
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Processes and encodes images in a given directory to the specified image format.
pub fn convert_images(
    pattern: &str,
    img_format: &ImageFormat,
    output: &Option<String>,
    option_lossless: &Option<bool>,
    option_quality: &Option<f32>,
    option_speed: &Option<u8>,
) -> Result<(), Error> {
    let paths: Vec<PathBuf> = glob::glob(pattern)?
        .filter_map(|entry| entry.ok())
        .collect();

    paths
        .par_iter()
        .filter(|path| is_supported(path, img_format))
        .try_for_each(|path| convert_image(path, img_format, output, option_lossless, option_quality, option_speed))?;

    Ok(())
}

/// Encodes an image to the specified image format and saves it to the specified output directory.
fn convert_image(
    input_path: &Path,
    img_format: &ImageFormat,
    output_dir: &Option<String>,
    option_lossless: &Option<bool>,
    option_quality: &Option<f32>,
    option_speed: &Option<u8>,
) -> Result<(), Error> {
    let image_reader = ImageReader::open(input_path)?;
    let image = image_reader.decode()?;

    let encode_lossless = option_lossless.unwrap_or(false);
    let encode_quality: f32 = option_quality.unwrap_or(90.);
    let encode_speed: u8 = option_speed.unwrap_or(3);

    let image_data = match img_format {
        ImageFormat::Webp => encode_webp(&image, encode_lossless, encode_quality)?,
        ImageFormat::Avif => encode_avif(&image, encode_quality, encode_speed, None, None)?,
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
