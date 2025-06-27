use crate::Error;
use image::{DynamicImage, EncodableLayout};
use crate::converter::DEPENDENCIES;

/// Provides encoder information
pub fn encoder_info() -> String {
    // we might have multiple versions of the package, use rfind to find the newest one
    let mut mozjpeg_version = "";
    match DEPENDENCIES.iter().rfind(|&&(name, _)| name == "mozjpeg") {
        Some((_name, version)) => {
            mozjpeg_version = version;
        }
        None => {
            println!("Package '{}' not found", "mozjpeg");
        }
    };

    format!(
        "Using \"mozjpeg\" ({})",
        mozjpeg_version
    )
}


/// Encodes a `DynamicImage` to bytes of webp format
pub fn encode_mozjpeg(image: &DynamicImage) -> Result<Vec<u8>, Error> {
    println!("jpeg1 {} {}", image.width(), image.height());
    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    println!("jpeg2 {} {}", image.width(), image.height());
    comp.set_size(image.width() as usize, image.height() as usize);
    println!("jpeg3 {} {}", image.width(), image.height());
    let mut comp = comp.start_compress(Vec::new())?;
    println!("jpeg4 {} {}", image.width(), image.height());
    comp.write_scanlines(image.to_rgb8().as_bytes())?; // this step seems to crash for many input files
    println!("jpeg5 {} {}", image.width(), image.height());
    Ok(comp.finish()?)
}
