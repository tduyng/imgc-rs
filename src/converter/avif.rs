use crate::Error;
use image::DynamicImage;
use ravif::*;
use rgb::FromSlice;
use crate::converter::DEPENDENCIES;

macro_rules! copy_enum_variants {
    ($name:ident, $($variant:ident),*) => {
        #[allow(missing_docs)]
        #[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Debug)]
        pub enum $name {
            $($variant),*
        }
    };
}

// re-imported enums from the image crates png encoder (so that they are usable in cli arguments)
copy_enum_variants!(BitDepth, Eight, Ten, Auto);
copy_enum_variants!(ColorModel, YCbCr, RGB);
copy_enum_variants!(AlphaColorMode, UnassociatedDirty, UnassociatedClean, Premultiplied);

fn convert_bit_depth_to_ext(bit_depth: Option<BitDepth>) -> ravif::BitDepth {
    match bit_depth.unwrap_or(BitDepth::Auto) {
        BitDepth::Eight => ravif::BitDepth::Eight,
        BitDepth::Ten => ravif::BitDepth::Ten,
        BitDepth::Auto => ravif::BitDepth::Auto
    }
}
fn convert_color_model_to_ext(color_model: Option<ColorModel>) -> ravif::ColorModel {
    match color_model.unwrap_or(ColorModel::YCbCr) {
        ColorModel::YCbCr => ravif::ColorModel::YCbCr,
        ColorModel::RGB => ravif::ColorModel::RGB
    }
}
fn convert_alpha_color_mode_to_ext(alpha_color_mode: Option<AlphaColorMode>) -> ravif::AlphaColorMode {
    match alpha_color_mode.unwrap_or(AlphaColorMode::UnassociatedClean) {
        AlphaColorMode::UnassociatedDirty => ravif::AlphaColorMode::UnassociatedDirty,
        AlphaColorMode::UnassociatedClean => ravif::AlphaColorMode::UnassociatedClean,
        AlphaColorMode::Premultiplied => ravif::AlphaColorMode::Premultiplied
    }
}

/// Provides encoder information
pub fn encoder_info(quality: f32, speed: u8,
                    bit_depth: Option<BitDepth>, color_model: Option<ColorModel>) -> String {
    // we have multiple ravif versions (one through image crate, one direct for the newest encoder version)
    //  with the implicit ordering through the build.rs generation we can use rfind to find the newest one
    let mut ravif_version = "";
    match DEPENDENCIES.iter().rfind(|&&(name, _)| name == "ravif") {
        Some((_name, version)) => {
            ravif_version = version;
        }
        None => {
            println!("Package '{}' not found", "ravif");
        }
    };
    
    format!(
        "Using \"ravif\" ({}) with options (quality: {}, speed: {}, bit depth: {:?}, color model: {:?})",
        ravif_version,
        quality,
        speed,
        convert_bit_depth_to_ext(bit_depth),
        convert_color_model_to_ext(color_model)
    )
}

/// Encodes a `DynamicImage` to bytes of avif format
pub fn encode_avif(image: &DynamicImage, quality: f32, speed: u8,
                   bit_depth: Option<BitDepth>, color_model: Option<ColorModel>,
                   alpha_color_mode: Option<AlphaColorMode>, alpha_quality: f32) -> Result<Vec<u8>, Error> {
    let avif_res: EncodedImage;
    if image.color().has_alpha() {
        let source_image = image.to_rgba8();
        let image = Img::new(source_image.as_rgba(), image.width() as usize, image.height() as usize);
        avif_res = Encoder::new()
            .with_quality(quality)
            .with_speed(speed) // speed: 1-10, 10 is fastest, but still slow
            .with_bit_depth(convert_bit_depth_to_ext(bit_depth))
            .with_internal_color_model(convert_color_model_to_ext(color_model))
            .with_alpha_quality(alpha_quality) // TODO: expose parameter
            .with_alpha_color_mode(convert_alpha_color_mode_to_ext(alpha_color_mode)) // internal ravif default
            .encode_rgba(image).expect("ERROR: could not convert screenshot bitmap to AVIF");
    } else {
        let source_image = image.to_rgb8();
        let image = Img::new(source_image.as_rgb(), image.width() as usize, image.height() as usize);
        avif_res = Encoder::new()
            .with_quality(quality)
            .with_speed(speed) // speed: 1-10, 10 is fastest, but still slow
            .with_bit_depth(convert_bit_depth_to_ext(bit_depth))
            .with_internal_color_model(convert_color_model_to_ext(color_model))
            .encode_rgb(image).expect("ERROR: could not convert screenshot bitmap to AVIF");
    }
    Ok(avif_res.avif_file)
}
