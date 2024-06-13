use crate::Error;
use image::DynamicImage;
use webp::Encoder;

/// Encodes a `DynamicImage` to bytes of webp format
pub fn encode_webp(image: &DynamicImage) -> Result<Vec<u8>, Error> {
    let encoder = Encoder::from_image(image).map_err(|e| Error::WebpError(e.to_string()))?;
    let webp_data = encoder.encode(100.0);
    Ok(webp_data.to_vec())
}
