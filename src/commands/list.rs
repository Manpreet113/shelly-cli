use crate::config::ConfigPaths;
use anyhow::Result;
use std::collections::HashSet;
use std::fs;

pub fn handle_list_wallpapers(paths: &ConfigPaths) -> Result<()> {
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

    for entry in fs::read_dir(&paths.user_wall_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let ext = path
                .extension()
                .map_or("".to_string(), |s| {
                    s.to_str().unwrap_or("").to_lowercase()
                });

            if image_exts.contains(ext.as_str()) || video_exts.contains(ext.as_str()) {
                println!("{}", path.to_string_lossy());
            }
        }
    }
    Ok(())
}