use crate::Error;
use image::DynamicImage;
use webp::{Encoder, WebPMemory};

/// Encodes a `DynamicImage` to bytes of webp format
pub fn encode_webp(image: &DynamicImage, lossless: bool, qualify: f32) -> Result<Vec<u8>, Error> {
    let encoder = Encoder::from_image(image).map_err(|e| Error::from_string(e.to_string()))?;
    let webp_data: WebPMemory = encoder.encode_simple(lossless, qualify).unwrap();
    Ok(webp_data.to_vec())
}
