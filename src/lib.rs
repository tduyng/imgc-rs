/*!
# Image Converter `imgc`

`imgc` is a command-line utility focusing on converting images into other formats,
 specifically focusing on support for modern image standards and encoders.

`imgc` simplifies the process of batch converting images,
 optimizing for both performance and storage efficiency.

*/

#![deny(missing_docs)]
/// Command-line interface functionality.
pub mod cli;
/// Image conversion functionality.
pub mod converter;
/// Error handling for the application.
mod error;
/// Image formats supported by the application.
pub mod format;

/// Utility functions and helpers.
pub mod utils;

pub use error::Error;
