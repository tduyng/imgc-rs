/// This module provides webp conversion via the webp crate
pub mod webp;
/// This module provides avif conversion via the ravif crate
pub mod avif;
/// This module provides webp conversion via the image crate
pub mod webp_image;
/// This module provides png conversion via the image crate
pub mod png;
mod mozjpeg;

use crate::{converter::webp::encode_webp, converter::avif::encode_avif, converter::png::encode_png, converter::webp_image::encode_webp_image, format::ImageFormat, utils::is_supported, Error};
use image::ImageReader;
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};
use std::collections::{LinkedList};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use bytesize::ByteSize;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use crate::converter::avif::{AlphaColorMode, BitDepth, ColorModel};
use crate::converter::mozjpeg::encode_mozjpeg;
use crate::converter::png::{CompressionType, FilterType};

// Include dependency version numbers
include!(concat!(env!("OUT_DIR"), "/versions.rs"));

/// Configuration parameters shared across all encoders
#[derive(Clone)]
pub struct CommonConfig {
    /// Glob pattern to match images to convert.
    /// Example: `images/**/*.png`
    pub pattern: String,

    /// Output directory (flat) of processed images.
    /// Defaults to the same location as the original images with the new file extension.
    pub output: String,

    /// Overwrite the existing output file if the current conversion resulted in a smaller file.
    /// Defaults to true.
    pub overwrite_if_smaller: bool,

    /// Overwrite existing outputs?
    /// Defaults to false. (Determined by filename match)
    pub overwrite_existing: bool,

    /// Discards the encoding result if it is larger than the input file (does not create an output file).
    /// Defaults to false.
    pub discard_if_larger_than_input: bool,
}

/// Processes and encodes images in a given directory to the specified image format.
pub fn convert_images(
    conf: CommonConfig,
    img_format: &ImageFormat,
    option_lossless: &Option<bool>,
    option_quality: &Option<f32>,
    option_speed: &Option<u8>,
    option_png_compression_type: &Option<CompressionType>,
    option_png_filter_type: &Option<FilterType>,
    option_avif_bit_depth: &Option<BitDepth>,
    option_avif_color_model: &Option<ColorModel>,
    option_avif_alpha_color_mode: &Option<AlphaColorMode>,
    option_avif_alpha_quality: &Option<f32>,
) -> Result<(), Error> {

    let mut paths: Vec<PathBuf> = glob::glob(&*conf.pattern)?
        .filter_map(|entry| entry.ok())
        // disable reading avif (FIXME: re-enable with reliable build+integration for reader)
        .filter(|path|
            (is_supported(path, img_format) && is_supported(path, &ImageFormat::Avif))
                || ImageFormat::from(path.as_path()) != ImageFormat::Unknown
        )
        .collect();
    paths.sort_by(|a,b| a.file_name().cmp(&b.file_name()));
    // TODO: check for collision candidates (same filename but different extensions => same encoded output filename format...)
    //  and come up with a solution

    if paths.is_empty() {
        println!("No images to convert, check input glob pattern and supported input formats.");
        return Ok(());
    }

    // create output directory if it does not exist
    if ! conf.output.is_empty() {
        let output_directory = Path::new(&conf.output);
        if ! fs::exists(output_directory)? {
            // is it possible to warn in docker if the target output directory is not host mounted?
            println!("Creating output directory \"{:?}\"", output_directory);
            fs::create_dir_all(output_directory).unwrap_or_else(|err| {
                eprintln!("Error creating the output directory: {err}");
                std::process::exit(1);
            });
        }
    }
    // IDEA: create output filename from configurable regex

    println!("Converting {} files...", paths.len());
    let encoder_data = match img_format {
        ImageFormat::Webp => webp::encoder_info(option_lossless.unwrap_or(false), option_quality.unwrap_or(90.)),
        ImageFormat::WebpImage => webp_image::encoder_info(),
        ImageFormat::Avif => avif::encoder_info(option_quality.unwrap_or(90.), option_speed.unwrap_or(3), None, None),
        ImageFormat::Png => png::encoder_info(),
        ImageFormat::Jpeg => mozjpeg::encoder_info(),
        _ => "unknown encoder".parse().unwrap(),
    };
    println!("{}", encoder_data);

    let global_stop = Arc::new(AtomicBool::new(false));
    let stop_signal = global_stop.clone();
    let mut ctrlc_counter = 0;
    ctrlc::set_handler(move || {
        if !global_stop.load(std::sync::atomic::Ordering::Relaxed) {
            println!("received Ctrl+C, stopping further queue processing!");
            global_stop.store(true, std::sync::atomic::Ordering::Relaxed);
        } else {
            println!("an encoding task is still active!{} processing will end afterwards.", str::repeat("!", ctrlc_counter));
        }
        ctrlc_counter += 1;
    }).expect("Error setting Ctrl-C handler");

    let style = ProgressStyle::with_template("[{elapsed_precise}/~{duration_precise} ({eta_precise} rem.)] {wide_bar:.cyan/blue} {pos:>7}/{len:7}").unwrap();
    let _results: LinkedList<(isize, usize, usize)> = paths.clone()
        .into_par_iter()
        .progress_with_style(style)
        .map(|path|
            if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
                return (-1, 0, 0);
            } else {
                 convert_image(
                     &*path, img_format,
                     conf.output.clone(), conf.overwrite_if_smaller,
                     conf.overwrite_existing, conf.discard_if_larger_than_input,
                     option_lossless, option_quality, option_speed,
                     option_png_compression_type, option_png_filter_type,
                     option_avif_bit_depth, option_avif_color_model, option_avif_alpha_color_mode, option_avif_alpha_quality
                 )
            }
            .map_err(|err| eprintln!("Failed to convert image {:?} : {:?}", path, err)).unwrap_or_else(|_| (-2, 0, 0))
        )
        .collect();

    let encode_successful = _results.par_iter()
        .filter(|(status, _, _)| *status == 0).count();
    let encode_skipped = _results.par_iter()
        .filter(|(status, _, _)| *status == -1).count();
    let encode_errors = _results.par_iter()
        .filter(|(status, _, _)| *status == -2).count();

    let total_input_size = _results.par_iter()
        .filter(|(status, _, _)| *status == 0 || *status == -1)
        .map(|(_, input_size, _)| input_size).sum::<usize>();
    let total_output_size = _results.par_iter()
        .filter(|(status, _, _)| *status == 0 || *status == -1)
        .map(|(_, _, output_size)| output_size).sum::<usize>();

    println!("Encode statistics:");
    println!("Successful: {:?}", encode_successful);
    println!("Skipped:    {:?}", encode_skipped);
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
    output: String,
    overwrite_if_smaller: bool,
    overwrite_existing: bool,
    discard_if_larger_than_input: bool,
    option_lossless: &Option<bool>,
    option_quality: &Option<f32>,
    option_speed: &Option<u8>,
    option_png_compression_type: &Option<CompressionType>,
    option_png_filter_type: &Option<FilterType>,
    option_avif_bit_depth: &Option<BitDepth>,
    option_avif_color_model: &Option<ColorModel>,
    option_avif_alpha_color_mode: &Option<AlphaColorMode>,
    option_avif_alpha_quality: &Option<f32>,
) -> Result<(isize, usize, usize), Error> {
    // returns tuple (status, input_size (B), output_size (B))
    let ext = img_format.extension();
    let output_path;
    if output.is_empty() {
        output_path = input_path.with_extension(ext)
    } else {
        output_path = Path::new(&output)
            .join(input_path.file_stem().unwrap())
            .with_extension(ext)
    };

    if fs::exists(output_path.clone())? && !overwrite_existing && !overwrite_if_smaller {
        // file exists, and we do not have any overwrite flag on? => return early
        //println!("skipped because output path exists and overwrite options are unset {}", input_path.display());
        return Ok((-1, 0, 0))
    }

    let image = ImageReader::open(input_path)?.decode()?;

    let encode_lossless = option_lossless.unwrap_or(false);
    let encode_quality: f32 = option_quality.unwrap_or(90.);
    let encode_speed: u8 = option_speed.unwrap_or(3);

    let image_data = match img_format {
        // TODO: more PNG lossless optimizers, jpeg xl
        ImageFormat::Webp => encode_webp(&image, encode_lossless, encode_quality)?,
        ImageFormat::WebpImage => encode_webp_image(&image)?,
        ImageFormat::Avif => encode_avif(
            &image, encode_quality, encode_speed,
            *option_avif_bit_depth, *option_avif_color_model,
            *option_avif_alpha_color_mode, option_avif_alpha_quality.unwrap_or(90.))?,
        ImageFormat::Png => encode_png(&image, *option_png_compression_type, *option_png_filter_type)?,
        ImageFormat::Jpeg => encode_mozjpeg(&image)?,
        _ => return Err(Error::from_string("Unsupported image format".to_string())),
    };

    let output_size =  image_data.len();
    if fs::exists(output_path.clone())? &&
        output_size >= fs::metadata(output_path.clone())?.len() as usize &&
        overwrite_if_smaller {
        // overwrite if smaller flag is on, but output exists and is already smaller than our encode
        //  => abort
        // TODO: how to propagate this information upwards into statistics?
        //println!(
        //    "skipped because output path exists,\
        //      overwrite_if_smaller is active,\
        //      but new output is larger than the existing one {}",
        //    input_path.display());
        return Ok((-1, 0, 0));
    }

    let input_size = fs::metadata(&input_path)?.len() as usize;
    if discard_if_larger_than_input && output_size >= input_size {
        // TODO: how to propagate this information upwards into statistics?
        //println!(
        //    "skipped because the output is larger than the input,\
        //      and discard_if_larger_than_input is active {}",
        //    input_path.display());
        return Ok((-1, 0, 0));
    }

    fs::write(output_path.clone(), image_data)?;
    Ok((0, input_size, output_size))
}
