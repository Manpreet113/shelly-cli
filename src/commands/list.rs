use crate::config::ConfigPaths;
use anyhow::Result;
use std::collections::HashSet;
use std::fs;

// `pub` so `main.rs` can call it.
pub fn handle_list_wallpapers(paths: &ConfigPaths) -> Result<()> {
    // These are the extensions from the original ListsCommand.hx
    let image_exts: HashSet<&str> = [
        "png", "jpg", "jpeg", "webp", "bmp", "gif", "tif", "tiff", "ico",
    ]
    .iter()
    .cloned()
    .collect();
    
    let video_exts: HashSet<&str> = [
        "mp4", "mkv", "webm", "avi", "mov", "flv", "wmv", "m4v",
    ]
    .iter()
    .cloned()
    .collect();

    // 1. Read the user's wallpaper directory
    for entry in fs::read_dir(&paths.user_wall_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // 2. Make sure it's a file
        if path.is_file() {
            // 3. Get the extension and lowercase it
            let ext = path
                .extension()
                .map_or("".to_string(), |s| {
                    s.to_str().unwrap_or("").to_lowercase()
                });

            // 4. Check if the extension is in either list
            if image_exts.contains(ext.as_str()) || video_exts.contains(ext.as_str()) {
                // 5. Print the full path, just like the original
                println!("{}", path.to_string_lossy());
            }
        }
    }
    Ok(())
}