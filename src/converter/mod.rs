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
use std::collections::LinkedList;
use indicatif::{ParallelProgressIterator, ProgressStyle};

/// Processes and encodes images in a given directory to the specified image format.
pub fn convert_images(
    pattern: &str,
    img_format: &ImageFormat,
    output: &Option<String>,
    overwrite_existing: &Option<bool>,
    option_lossless: &Option<bool>,
    option_quality: &Option<f32>,
    option_speed: &Option<u8>,
) -> Result<(), Error> {
    let mut paths: Vec<PathBuf> = glob::glob(pattern)?
        .filter_map(|entry| entry.ok())
        .collect();
    paths.sort_by(|a,b| a.file_name().cmp(&b.file_name()));
    
    let style = ProgressStyle::with_template("[{elapsed_precise}/~{duration_precise} ({eta_precise} rem.)] {wide_bar:.cyan/blue} {pos:>7}/{len:7}").unwrap();
    let _results: LinkedList<Vec<()>> = paths
        .into_par_iter()
        .progress_with_style(style)
        .filter(|path| is_supported(path, img_format))
        .map(|path| convert_image(&*path, img_format, output, overwrite_existing, option_lossless, option_quality, option_speed)
            .map_err(|err| eprintln!("Failed to convert image {:?} : {:?}", path, err)).unwrap()
        )
        .collect_vec_list();
    
    // TODO: encoding statistics
    Ok(())
}

/// Encodes an image to the specified image format and saves it to the specified output directory.
fn convert_image(
    input_path: &Path,
    img_format: &ImageFormat,
    output_dir: &Option<String>,
    overwrite_existing: &Option<bool>,
    option_lossless: &Option<bool>,
    option_quality: &Option<f32>,
    option_speed: &Option<u8>,
) -> Result<(), Error> {
    let ext = img_format.extension();
    let output_path = if let Some(output_dir) = output_dir {
        Path::new(&output_dir)
            .join(input_path.file_stem().unwrap())
            .with_extension(ext)
    } else {
        input_path.with_extension(ext)
    };
    
    if fs::exists(output_path.clone())? && !overwrite_existing.unwrap_or(false) {
        // file exists, and we do not have the overwrite flag on? => return early
        return Ok(())
    }

    let image_reader = ImageReader::open(input_path)?;
    let image = image_reader.decode()?;

    let encode_lossless = option_lossless.unwrap_or(false);
    let encode_quality: f32 = option_quality.unwrap_or(90.);
    let encode_speed: u8 = option_speed.unwrap_or(3);
    
    let image_data = match img_format {
        // TODO: PNG lossless optimizer, image-rs webp encoder path
        ImageFormat::Webp => encode_webp(&image, encode_lossless, encode_quality)?,
        ImageFormat::Avif => encode_avif(&image, encode_quality, encode_speed, None, None)?,
        _ => return Err(Error::from_string("Unsupported image format".to_string())),
    };

    fs::write(output_path.clone(), image_data)?;
    //println!("Generated: {}", output_path.display());

    Ok(())
}
