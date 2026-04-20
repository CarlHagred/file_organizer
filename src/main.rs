use directories::ProjectDirs;
use serde_json::from_str;
use std::collections::HashMap;
use std::env::args;
use std::fs::{create_dir_all, read_dir, read_to_string, rename, write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extention_config = read_to_string("./config.json")?;
    let extention_map: HashMap<String, String> = from_str(&extention_config)?;
    let argument = match args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("A path needs to be provided as the first argument");
            return Ok(());
        }
    };
    let path_content = read_dir(&argument)?;
    let base_dir = std::path::Path::new(&argument);
    for content in path_content {
        let entry = content?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(ext_str) = path.extension().and_then(|ext| ext.to_str()) {
            let target_folder = match extention_map.get(ext_str) {
                Some(target_folder) => target_folder,
                None => continue,
            };
            let target_dir_path = base_dir.join(target_folder);
            create_dir_all(&target_dir_path)?;
            let file_name = match path.file_name() {
                Some(file_name) => file_name,
                None => {
                    println!("Could not extract filename from {}", path.display());
                    return Ok(());
                }
            };
            let destination_path = target_dir_path.join(file_name);
            rename(&path, destination_path)?;
            println!("Moved {} to {}", file_name.to_string_lossy(), target_folder);
        }
    }
    Ok(())
}
