use clap::{ArgAction, Parser, Subcommand};

/// Image converter CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Glob pattern to match images to convert.
    /// Example: `images/**/*.png`
    pub pattern: String,

    /// Output directory (flat) of processed images.
    /// Defaults to the same location as the original images with the new file extension.
    #[clap(short, long, default_value = None)]
    pub output: Option<String>,

    /// Overwrite the existing output file if the current conversion resulted in a smaller file.
    #[clap(long, action = Some(ArgAction::SetTrue))]
    pub overwrite_if_smaller: Option<bool>,

    /// Overwrite existing output files regardless of size.
    #[clap(long, action = Some(ArgAction::SetTrue))]
    pub overwrite_existing: Option<bool>,

    /// Discards the encoding result if it is larger than the input file (does not create an output file).
    #[clap(long, action = Some(ArgAction::SetTrue))]
    pub discard_if_larger_than_input: Option<bool>,

    /// The command to execute.
    #[command(subcommand)]
    /// The available commands are `webp`, `webp-image`, `avif`, `png` and `clean`.
    pub command: Command,
}

/// Image converter actions
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Convert images to webp format (using webp crate)
    Webp {
        /// Use lossless encoding mode. Defaults to false.
        #[clap(long, action = Some(ArgAction::SetTrue))]
        lossless: Option<bool>,

        /// Control target quality (0 - 100, lower is worse but results in smaller files).
        /// Defaults to 90.0.
        #[clap(short, long)]
        quality: Option<f32>,
    },
    
    /// Convert images to webp format (using image crate)
    WebpImage {}, // only lossless is available, no configuration parameters
    
    /// Convert images to avif format (using ravif crate)
    Avif {
        /// Control target quality (0 - 100, lower is worse but results in smaller files).
        /// Defaults to 90.0.
        #[clap(short, long)]
        quality: Option<f32>,

        /// Control encoding speed (1 - 10, lower is much slower but has a better quality and lower filesize).
        /// Defaults to 3.
        #[clap(short, long)]
        speed: Option<u8>,
        
        /// Choose internal bit depth. (in the generated avif file, nothing to do with the input file)
        #[clap(long, value_enum)]
        bit_depth: Option<crate::converter::avif::BitDepth>,
        
        /// Choose internal color model. (in the generated avif file, nothing to do with the input file)
        #[clap(long, value_enum)]
        color_model: Option<crate::converter::avif::ColorModel>,
        
        /// Choose internal alpha color mode. (in the generated avif file, nothing to do with the input file)
        /// Irrelevant for images without transparency.
        #[clap(long, value_enum)]
        alpha_color_mode: Option<crate::converter::avif::AlphaColorMode>,
        
        /// Control target alpha quality (0 - 100, lower is worse).
        /// Defaults to 90.0.
        #[clap(short, long)]
        alpha_quality: Option<f32>,
    },
    
    /// Convert images to png format (using image crate)
    Png {
        /// Choose the png compression type
        ///
        /// See: https://docs.rs/image/latest/image/codecs/png/enum.CompressionType.html
        #[clap(long, value_enum)]
        compression_type: Option<crate::converter::png::CompressionType>,
        
        /// Choose the png filter type
        ///
        /// See: https://docs.rs/image/latest/image/codecs/png/enum.CompressionType.html
        #[clap(long, value_enum)]
        filter_type: Option<crate::converter::png::FilterType>,
    },

    /// Convert images to optimized jpeg format (using mozjpeg crate)
    Jpeg {},

    /// Remove files matching a glob pattern
    Clean {
        /// Glob pattern to match files to remove.
        pattern: String,
    },
}
