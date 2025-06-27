use clap::Parser;
use imgc::{
    cli::{CliArgs, Command},
    converter::convert_images,
    format::ImageFormat,
    utils::remove_files,
    Error,
};
use imgc::converter::CommonConfig;

fn main() -> Result<(), Error> {
    let args = CliArgs::parse();
    let conf = CommonConfig::from(
        CommonConfig { 
            pattern: args.pattern,
            output: args.output.unwrap_or("".parse().unwrap()),
            overwrite_if_smaller: args.overwrite_if_smaller.unwrap(),
            overwrite_existing: args.overwrite_existing.unwrap(),
            discard_if_larger_than_input: args.discard_if_larger_than_input.unwrap(),
        }
    );
    match args.command {
        Command::Webp { lossless, quality}
            => convert_images(conf, &ImageFormat::Webp, &lossless, &quality, &None, &None, &None, &None, &None, &None, &None)?,
        Command::Avif { quality, speed, bit_depth, color_model, alpha_color_mode, alpha_quality}
            => convert_images(conf, &ImageFormat::Avif, &None, &quality, &speed, &None, &None, &bit_depth, &color_model, &alpha_color_mode, &alpha_quality)?,
        Command::WebpImage {}
            => convert_images(conf, &ImageFormat::WebpImage, &None, &None, &None, &None, &None, &None, &None, &None, &None)?,
        Command::Png { compression_type, filter_type }
            => convert_images(conf, &ImageFormat::Png, &None, &None, &None, &compression_type, &filter_type, &None, &None, &None, &None)?,
        Command::Jpeg {}
            => convert_images(conf, &ImageFormat::Jpeg, &None, &None, &None, &None, &None, &None, &None, &None, &None)?,
        Command::Clean { pattern } => remove_files(&pattern)?,
    }
    Ok(())
}
