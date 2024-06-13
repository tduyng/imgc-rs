/// Represents an error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Represents an IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Represents a formatting error.
    #[error("Fmt error: {0}")]
    FmtError(#[from] std::fmt::Error),

    /// Represents a glob error.
    #[error("Glob error: {0}")]
    GlobError(#[from] glob::GlobError),

    /// Represents a pattern error.
    #[error("Pattern error: {0}")]
    PatternError(#[from] glob::PatternError),

    /// Represents a clap error.
    #[error("Clap error: {0}")]
    ClapError(#[from] clap::Error),

    /// Represents an image error.
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    /// Represents a Webp error.
    #[error("Webp error: {0}")]
    WebpError(String),

    /// Represents an unsupported format error.
    #[error("Unsupported format")]
    UnsupportedFormat,
}
