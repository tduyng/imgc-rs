use std::path::Path;

/// `ImageFormat` defines the various formats that an image can have.
///
/// This enumeration covers a wide range of common and less common image formats.
/// Each variant represents a different format that an image file can be encoded in.
/// The `Other` variant is provided to allow for formats not explicitly listed here,
/// enabling the enumeration to hold any format specified as a `String`.
///
/// # Examples
///
/// ```
/// use your_crate::ImageFormat;
///
/// let format = ImageFormat::Png;
/// let unknown_format = ImageFormat::Other("custom-format".to_string());
/// ```
#[derive(Debug, PartialEq)]
pub enum ImageFormat {
    /// AV1 Image File Format, a format designed for high compression efficiency.
    Avif,

    /// Bitmap, a raster graphics image file format used to store bitmap digital images.
    Bmp,

    /// DirectDraw Surface, a container format for storing data compressed with the S3TC algorithm.
    Dds,

    /// Farbfeld, a simple image file format designed to work well for lossless compression.
    Farbfeld,

    /// Graphics Interchange Format, a bitmap image format that supports animation.
    Gif,

    /// High Dynamic Range Image File Format, a raster graphics file format for high dynamic range images.
    Hdr,

    /// Icon, a bitmap image format used for icons in Microsoft Windows.
    Ico,

    /// Joint Photographic Experts Group, an image compression standard that supports lossy and lossless compression.
    Jpeg,

    /// OpenEXR, a high dynamic range raster file format.
    Exr,

    /// Portable Network Graphics, a raster graphics file format that supports lossless data compression.
    Png,

    /// Portable anymap, a family of file formats to store bitmap images.
    Pnm,

    /// QuickTime Image, a raster graphics file format used by Apple's QuickTime framework.
    Qoi,

    /// Truevision TGA, a raster graphics file format used for storing images.
    Tga,

    /// Tagged Image File Format, a file format for storing raster graphics images.
    Tiff,

    /// WebP, an image format that provides lossless and lossy compression for images on the web.
    Webp,
    /// WebP, but encoded with the lossless VP8L encoder from image crate
    WebpImage,

    /// Represents an image format not explicitly listed here.
    Unknown,
}

impl ImageFormat {
    /// Get the file extension associated with the image format
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Avif => "avif",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Dds => "dds",
            ImageFormat::Farbfeld => "ff",
            ImageFormat::Gif => "gif",
            ImageFormat::Hdr => "hdr",
            ImageFormat::Ico => "ico",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Exr => "exr",
            ImageFormat::Png => "png",
            ImageFormat::Pnm => "pnm",
            ImageFormat::Qoi => "qoi",
            ImageFormat::Tga => "tga",
            ImageFormat::Tiff => "tiff",
            ImageFormat::Webp => "webp",
            ImageFormat::WebpImage => "webp",
            ImageFormat::Unknown => "?",
        }
    }

    /// Determine the image format based on the file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_ascii_lowercase().as_str() {
            "avif" => ImageFormat::Avif,
            "bmp" => ImageFormat::Bmp,
            "dds" => ImageFormat::Dds,
            "ff" | "farbfeld" => ImageFormat::Farbfeld,
            "gif" => ImageFormat::Gif,
            "hdr" => ImageFormat::Hdr,
            "ico" => ImageFormat::Ico,
            "jpeg" | "jpg" => ImageFormat::Jpeg,
            "exr" => ImageFormat::Exr,
            "png" => ImageFormat::Png,
            "pnm" => ImageFormat::Pnm,
            "qoi" => ImageFormat::Qoi,
            "tga" => ImageFormat::Tga,
            "tiff" | "tif" => ImageFormat::Tiff,
            "webp" => ImageFormat::Webp,
            _ => ImageFormat::Unknown,
        }
    }
}

impl From<&Path> for ImageFormat {
    fn from(path: &Path) -> Self {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ImageFormat::from_extension(ext),
            None => ImageFormat::Unknown,
        }
    }
}
