use image::{codecs::webp::WebPEncoder, DynamicImage, ExtendedColorType};

pub fn to_webp(image: &DynamicImage) -> Result<Vec<u8>, String> {
    let rgba_image = image.to_rgba8();
    let mut webp_data = Vec::new();
    let encoder = WebPEncoder::new_lossless(&mut webp_data);
    encoder
        .encode(
            rgba_image.as_raw(),
            rgba_image.width(),
            rgba_image.height(),
            ExtendedColorType::Rgba8,
        )
        .map_err(|e| format!("{e}"))?;

    Ok(webp_data)
}
