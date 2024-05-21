use image::{io::Reader, DynamicImage};
use rayon::prelude::*;
use webp::{Encoder, WebPMemory};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Converts images in a given directory to the WebP format.
///
/// # Arguments
///
/// * `path` - A reference to the path of the directory containing images to convert.
/// * `output_dir` - An optional output directory where the converted images will be saved. If `None`, the converted images will be saved in the same directory as the original images.
///
/// # Returns
///
/// * `Result<(), String>` - An empty result if successful, or an error message if an error occurs.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let result = convert_images(Path::new("images"), &Some(String::from("output")));
/// if let Err(e) = result {
///     eprintln!("Error: {}", e);
/// }
/// ```
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

/// Checks if the given path points to a supported image format.
///
/// # Arguments
///
/// * `path` - A reference to the path of the image file.
///
/// # Returns
///
/// * `bool` - `true` if the image format is supported and not already a WebP, `false` otherwise.
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

/// Converts an image to the WebP format and saves it to the specified output directory.
///
/// # Arguments
///
/// * `input_path` - A reference to the path of the input image file.
/// * `output_dir` - An optional output directory where the converted image will be saved. If `None`, the converted image will be saved in the same directory as the original image.
///
/// # Returns
///
/// * `Result<(), String>` - An empty result if successful, or an error message if an error occurs.
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

    fs::write(output_path.clone(), &webp_data.to_vec())
        .map_err(|e| format!("Failed to write WebP file: {}", e))?;

    println!("Generated: {}", output_path.display());

    Ok(())
}

/// Encodes a `DynamicImage` to WebP format.
///
/// # Arguments
///
/// * `image` - A reference to the `DynamicImage` to encode.
///
/// # Returns
///
/// * `Result<WebPMemory, String>` - The encoded WebP image data, or an error message if an error occurs.
fn to_webp(image: &DynamicImage) -> Result<WebPMemory, String> {
       let encoder = Encoder::from_image(image)
           .map_err(|e| format!("Failed to create a webp encoder: {}", e))?;
    let webp_data = encoder.encode(100.0);
       Ok(webp_data)
}