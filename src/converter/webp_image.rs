use crate::Error;
use image::{DynamicImage, ImageEncoder};
use crate::converter::DEPENDENCIES;

/// Provides encoder information
pub fn encoder_info() -> String {
    // we might have multiple versions of the package, use rfind to find the newest one
    let mut image_version = "";
    match DEPENDENCIES.iter().rfind(|&&(name, _)| name == "image") {
        Some((_name, version)) => {
            image_version = version;
        }
        None => {
            println!("Package '{}' not found", "image");
        }
    };

    format!(
        "Using \"webp (from image crate)\" ({})",
        image_version
    )
}


/// Encodes a `DynamicImage` to bytes of webp format
pub fn encode_webp_image(image: &DynamicImage) -> Result<Vec<u8>, Error> {
    let mut output = Vec::new();
    if image.color().has_alpha() {
        let source_image = image.to_rgba8();
        image::codecs::webp::WebPEncoder::new_lossless(&mut output)
            .write_image(
                source_image.as_ref(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgba8,
            )?;
    } else {
        let source_image = image.to_rgb8();
        image::codecs::webp::WebPEncoder::new_lossless(&mut output)
            .write_image(
                source_image.as_ref(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgb8,
            )?;
    }
    Ok(output)
}
