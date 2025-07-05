use crate::Error;
use image::DynamicImage;
use webp::{Encoder, WebPMemory};
use crate::converter::DEPENDENCIES;

/// Provides encoder information
pub fn encoder_info(lossless: bool, qualify: f32) -> String {
    // we might have multiple versions of the package, use rfind to find the newest one
    let mut webp_version = "";
    match DEPENDENCIES.iter().rfind(|&&(name, _)| name == "webp") {
        Some((_name, version)) => {
            webp_version = version;
        }
        None => {
            println!("Package '{}' not found", "webp");
        }
    };

    format!(
        "Using \"webp\" ({}) with options (lossless: {}, qualify: {})",
        webp_version,
        lossless,
        qualify
    )
}


/// Encodes a `DynamicImage` to bytes of webp format
pub fn encode_webp(image: &DynamicImage, lossless: bool, qualify: f32) -> Result<Vec<u8>, Error> {
    let encoder = Encoder::from_image(image).map_err(|e| Error::from_string(e.to_string()))?;
    let webp_data: WebPMemory = encoder.encode_simple(lossless, qualify).unwrap();
    Ok(webp_data.to_vec())
}
