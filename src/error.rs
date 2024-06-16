use std::error::Error as StdError;
use std::fmt;

/// Represents the error type for the application.
pub struct Error {
    inner: Box<dyn StdError + Send + Sync>,
}

impl Error {
    /// Constructs a new `Error` from any type that implements `std::error::Error`.
    pub fn new<E: StdError + Send + Sync + 'static>(error: E) -> Error {
        Error {
            inner: Box::new(error),
        }
    }

    /// Constructs a new `Error` from a string message.
    pub fn from_string(msg: String) -> Self {
        Self::new(CustomError(msg))
    }

    /// Helper function to get the underlying cause of the error.
    pub fn cause(&self) -> &(dyn StdError + 'static) {
        &*self.inner
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source()
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::new(err)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Self::new(err)
    }
}

impl From<glob::GlobError> for Error {
    fn from(err: glob::GlobError) -> Self {
        Self::new(err)
    }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Self {
        Self::new(err)
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Self::new(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Self::new(err)
    }
}

/// Custom error type for wrapping a string message.
struct CustomError(String);

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

impl StdError for CustomError {}
