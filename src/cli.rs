use clap::Parser;

/// Image converter CLI
#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Directory containing images to process.
    #[clap(short, long)]
    pub dir: String,

    /// Directory to save processed images. Defaults to the same location as the original images.
    #[clap(short, long)]
    pub output: Option<String>,
}
