use crate::Error;
use image::{DynamicImage, ImageEncoder};
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
copy_enum_variants!(CompressionType, Default, Fast, Best);
copy_enum_variants!(FilterType, NoFilter, Sub, Up, Avg, Paeth, Adaptive);

fn convert_compression_type_to_ext(compression_type: Option<CompressionType>) -> image::codecs::png::CompressionType {
    match compression_type.unwrap_or(CompressionType::Default) {
        CompressionType::Default => image::codecs::png::CompressionType::Default,
        CompressionType::Fast => image::codecs::png::CompressionType::Fast,
        CompressionType::Best => image::codecs::png::CompressionType::Best
    }
}
fn convert_filter_type_to_ext(filter_type: Option<FilterType>) -> image::codecs::png::FilterType {
    match filter_type.unwrap_or(FilterType::Adaptive) {
        FilterType::NoFilter => image::codecs::png::FilterType::NoFilter,
        FilterType::Sub => image::codecs::png::FilterType::Sub,
        FilterType::Up => image::codecs::png::FilterType::Up,
        FilterType::Avg => image::codecs::png::FilterType::Avg,
        FilterType::Paeth => image::codecs::png::FilterType::Paeth,
        FilterType::Adaptive => image::codecs::png::FilterType::Adaptive,
    }
}

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
        "Using \"png (from image crate)\" ({})",
        image_version
    )
}


/// Encodes a `DynamicImage` to bytes of webp format
pub fn encode_png(image: &DynamicImage, compression_type: Option<CompressionType>, filter_type: Option<FilterType>) -> Result<Vec<u8>, Error> {
    let mut output = Vec::new();
    let ext_compression_type = convert_compression_type_to_ext(compression_type);// default is fast
    let ext_filter_type = convert_filter_type_to_ext(filter_type); // default is adaptive
    if image.color().has_alpha() {
        let source_image = image.to_rgba8();
        image::codecs::png::PngEncoder::new_with_quality(&mut output, ext_compression_type, ext_filter_type)
            .write_image(
                source_image.as_ref(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgba8,
            )?;
    } else {
        let source_image = image.to_rgb8();
        image::codecs::png::PngEncoder::new_with_quality(&mut output, ext_compression_type, ext_filter_type)
            .write_image(
                source_image.as_ref(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgb8,
            )?;
    }
    Ok(output)
}
