use clap::{ArgAction, Parser, Subcommand};

/// Image converter CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The command to execute.
    #[command(subcommand)]
    /// The available commands are `Webp`, `Avif` and `Clean`.
    pub command: Command,
}

/// Image converter actions
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Convert images to webp format
    Webp {
        /// Glob pattern to match images to convert.
        /// Example: `images/**/*.png`
        pattern: String,

        /// (Optional) Output of processed images. Defaults to the same location as the original images.
        #[clap(short, long)]
        output: Option<String>,

        /// (Optional) Overwrite existing outputs? Defaults to false. (Determined by filename match)
        #[clap(long, action = Some(ArgAction::SetTrue))]
        overwrite_existing: Option<bool>,

        /// (Optional) Use lossless encoding mode. Defaults to false.
        #[clap(long, action = Some(ArgAction::SetTrue))]
        lossless: Option<bool>,

        /// (Optional) Control target quality for encoding (0 - 100, lower is worse). Defaults to 90.0.
        #[clap(short, long)]
        quality: Option<f32>,
    },
    /// Convert images to avif format
    Avif {
        /// Glob pattern to match images to convert.
        /// Example: `images/**/*.png`
        pattern: String,

        /// (Optional) Output of processed images. Defaults to the same location as the original images.
        #[clap(short, long)]
        output: Option<String>,

        /// (Optional) Overwrite existing outputs? Defaults to false. (Determined by filename match)
        #[clap(long, action = Some(ArgAction::SetTrue))]
        overwrite_existing: Option<bool>,

        /// (Optional) Control target quality for encoding (0 - 100, lower is worse). Defaults to 90.0.
        #[clap(short, long)]
        quality: Option<f32>,

        /// (Optional) Control encoding speed (1 - 10, lower is much slower but has a better quality and lower filesize). Defaults to 3.
        #[clap(short, long)]
        speed: Option<u8>,
    },

    /// Remove files matching a glob pattern
    Clean {
        /// Glob pattern to match files to remove.
        pattern: String,
    },
}
