use crate::config::ConfigPaths;
use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::prefs::handle_prefs_set;

pub fn handle_wallpaper(paths: &ConfigPaths, path: &Path, no_scheme: bool) -> Result<()> {
    if !path.exists() {
        bail!("File does not exist: {}", path.to_string_lossy());
    }

    let video_exts: HashSet<&str> = ["mp4", "mkv", "webm", "avi", "mov"].iter().cloned().collect();
    let is_video = path.extension().map_or(false, |ext| video_exts.contains(ext.to_str().unwrap_or("")));

    let image_for_colors = if is_video {
        println!("Extracting frame from video...");
        let status = Command::new("ffmpeg")
            .args([
                "-ss", "20",
                "-i", path.to_str().unwrap(),
                "-vframes:v", "1",
                "-y", // Overwrite
                "/tmp/shelly-color-gen.png",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("Failed to run ffmpeg. Is it installed?")?;

        if !status.success() {
            bail!("ffmpeg command failed");
        }
        PathBuf::from("/tmp/shelly-color-gen.png")
    } else {
        path.to_path_buf()
    };

    if !no_scheme {
        println!("Generating color schemes with matugen...");
        let status = Command::new("matugen")
            .args([
                "image", image_for_colors.to_str().unwrap(),
                "-m", "dark", 
                "-j", "hex",
            ])
            .status()
            .context("Failed to run matugen. Is it installed?")?;
        
        if !status.success() {
            bail!("matugen command failed");
        }
    } else {
        println!("--no-scheme-gen passed, skipping color generation.");
    }
    
    if is_video {
        fs::remove_file(&image_for_colors)?;
    }
    
    println!("Updating preferences...");
    handle_prefs_set(
        paths,
        "theme.wallpaper",
        path.to_str().context("Wallpaper path is not valid UTF-8")?,
    )?;

    println!("Wallpaper successfully changed!");
    Ok(())
}