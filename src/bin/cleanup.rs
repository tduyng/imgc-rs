use std::{fs, path::Path};

fn main(){
    let target_dir = "examples";

    if let Err(e) = remove_webp_files(Path::new(target_dir)) {
        eprintln!("Error: {}", e);
    }
}

fn remove_webp_files(dir: &Path) -> Result<(), String> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).map_err(|e| format!("Read directory failed: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                remove_webp_files(&path)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("webp") {
                fs::remove_file(&path)
                    .map_err(|e| format!("Failed to delete file {}: {}", path.display(), e))?;
                println!("Deleted: {}", path.display());
            }
        }
    }

    Ok(())
}
