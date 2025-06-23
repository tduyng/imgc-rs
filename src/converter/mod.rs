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
use bytesize::ByteSize;
use indicatif::{ParallelProgressIterator, ProgressStyle};

// Include dependency version numbers
include!(concat!(env!("OUT_DIR"), "/versions.rs"));

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

    let encoder_data = match img_format {
        // TODO: PNG lossless optimizer, image-rs webp encoder path
        ImageFormat::Webp => webp::encoder_info(option_lossless.unwrap_or(false), option_quality.unwrap_or(90.)),
        ImageFormat::Avif => avif::encoder_info(option_quality.unwrap_or(90.), option_speed.unwrap_or(3), None, None),
        _ => "unknown encoder".parse().unwrap(),
    };

    println!("Converting {} files...", paths.len());
    println!("{}", encoder_data);

    let style = ProgressStyle::with_template("[{elapsed_precise}/~{duration_precise} ({eta_precise} rem.)] {wide_bar:.cyan/blue} {pos:>7}/{len:7}").unwrap();
    let _results: LinkedList<(isize, usize, usize)> = paths.clone()
        .into_par_iter()
        .progress_with_style(style)
        .filter(|path| is_supported(path, img_format))
        .map(|path| convert_image(&*path, img_format, output, overwrite_existing, option_lossless, option_quality, option_speed)
            .map_err(|err| eprintln!("Failed to convert image {:?} : {:?}", path, err)).unwrap_or_else(|_| (-2, 0, 0))
        )
        .collect();

    let encode_successful = _results.par_iter().filter(|(status, _, _)| *status == 0).count();
    let encode_existing = _results.par_iter().filter(|(status, _, _)| *status == -1).count();
    let encode_errors = _results.par_iter().filter(|(status, _, _)| *status == -2).count();
    let total_input_size = _results.par_iter().filter(|(status, _, _)| *status == 0 || *status == -1)
        .map(|(_, input_size, _)| input_size).sum::<usize>();
    let total_output_size = _results.par_iter().filter(|(status, _, _)| *status == 0 || *status == -1)
        .map(|(_, _, output_size)| output_size).sum::<usize>();
    println!("Encode statistics:");
    println!("Successful: {:?}", encode_successful);
    println!("Skipped:    {:?}", encode_existing);
    println!("Errors:     {:?}", encode_errors);
    if total_input_size > 0 && total_output_size > 0 {
        println!("Total input size:  {}", ByteSize::b(total_input_size as u64));
        println!("Total output size: {}", ByteSize::b(total_output_size as u64));
        println!("Compression ratio: {:.02}%", total_output_size as f64 / total_input_size as f64 * 100.0);
    } else {
        println!("Input and output size could not be determined, please try using OS-native binaries.")
    }
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
) -> Result<(isize, usize, usize), Error> {
    // returns tuple (status, input_size (B), output_size (B))
    let ext = img_format.extension();
    let output_path = if let Some(output_dir) = output_dir {
        Path::new(&output_dir)
            .join(input_path.file_stem().unwrap())
            .with_extension(ext)
    } else {
        input_path.with_extension(ext)
    };
    let input_size = fs::metadata(&input_path)?.len();
    
    if fs::exists(output_path.clone())? && !overwrite_existing.unwrap_or(false) {
        // file exists, and we do not have the overwrite flag on? => return early
        return Ok((-1, input_size as usize, fs::metadata(&output_path)?.len() as usize))
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
    let output_size =  image_data.len();
    fs::write(output_path.clone(), image_data)?;

    Ok((0, input_size as usize, output_size))
}
