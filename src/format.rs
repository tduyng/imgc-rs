use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum ImageFormat {
    Avif,
    Bmp,
    Dds,
    Farbfeld,
    Gif,
    Hdr,
    Ico,
    Jpeg,
    Exr,
    Png,
    Pnm,
    Qoi,
    Tga,
    Tiff,
    Webp,
    Other(String),
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
            ImageFormat::Other(ext) => ext,
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
            other => ImageFormat::Other(other.to_string()),
        }
    }
}

impl From<&Path> for ImageFormat {
    fn from(path: &Path) -> Self {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ImageFormat::from_extension(ext),
            None => ImageFormat::Other("unknown".to_string()),
        }
    }
}
