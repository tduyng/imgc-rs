use clap::{Parser, Subcommand};

/// Image converter CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The command to execute.
    #[command(subcommand)]
    /// The available commands are `Webp` and `Clean`.
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
    },

    /// Remove files matching a glob pattern
    Clean {
        /// Glob pattern to match files to remove.
        pattern: String,
    },
}
