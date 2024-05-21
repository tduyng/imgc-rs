use std::{fs, path::Path};

use crate::format::ImageFormat;

pub fn is_supported(path: &Path, ignore_format: &ImageFormat) -> bool {
    if let Some(extension) = path.extension() {
        if extension == ignore_format.extension() {
            return false;
        }
    }

    match fs::read(path) {
        Ok(data) => image::guess_format(&data).is_ok(),
        Err(_) => false,
    }
}

pub fn remove_files(dir: &Path, ext: &str) -> Result<(), String> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).map_err(|e| format!("Read directory failed: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                remove_files(&path, ext)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some(ext) {
                fs::remove_file(&path)
                    .map_err(|e| format!("Failed to delete file {}: {}", path.display(), e))?;
                println!("Deleted: {}", path.display());
            }
        }
    }

    Ok(())
}
