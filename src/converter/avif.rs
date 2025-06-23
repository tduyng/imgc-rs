use crate::Error;
use image::DynamicImage;
use ravif::*;
use ravif::ColorModel::RGB;
use rgb::FromSlice;

/// Encodes a `DynamicImage` to bytes of avif format
pub fn encode_avif(image: &DynamicImage, quality: f32, speed: u8,
                   bit_depth: Option<BitDepth>, interal_color_model: Option<ColorModel>) -> Result<Vec<u8>, Error> {
    // TODO: handle bit-depth and alpha determination automatically...
    let source_image = image.to_rgb8();
    let image = Img::new(source_image.as_rgb(), image.width() as usize, image.height() as usize);
    let avif_res = Encoder::new()
        .with_quality(Option::from(quality).unwrap_or(90.))
        .with_speed(Option::from(speed).unwrap_or(4)) // speed: 1-10, 10 is fastest, but still slow
        .with_bit_depth(bit_depth.unwrap_or(BitDepth::Eight))
        .with_internal_color_model(interal_color_model.unwrap_or(RGB))
        .encode_rgb(image).expect("ERROR: could not convert screenshot bitmap to AVIF");
    Ok(avif_res.avif_file)
}
