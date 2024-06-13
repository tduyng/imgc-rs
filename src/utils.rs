use std::{fs, path::Path};

use crate::{format::ImageFormat, Error};

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

pub fn remove_files(dir: &Path, ext: &str) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                remove_files(&path, ext)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some(ext) {
                fs::remove_file(&path)?;
                println!("Deleted: {}", path.display());
            }
        }
    }

    Ok(())
}
