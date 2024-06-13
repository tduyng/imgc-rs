#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Fmt error: {0}")]
    FmtError(#[from] std::fmt::Error),

    #[error("Clap error: {0}")]
    ClapError(#[from] clap::Error),

    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Webp error: {0}")]
    WebpError(String),

    #[error("Unsupported format")]
    UnsupportedFormat,
}
