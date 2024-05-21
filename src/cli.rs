use clap::{Parser, Subcommand};

/// Image converter CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

/// Image converter actions
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Convert images to webp format
    Webp {
        /// (Required) Directory containing images to process.
        #[clap(short, long)]
        dir: String,

        /// (Optional) Output of processed images. Defaults to the same location as the original images.
        #[clap(short, long)]
        output: Option<String>,
    },

    /// Clean files by given extension
    Clean {
        /// (Required) Directory to clean files
        #[clap(short, long)]
        dir: String,

        /// (Required) Extension files to clean
        #[clap(short, long)]
        ext: String,
    },
}
