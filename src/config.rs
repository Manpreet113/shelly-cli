use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// A struct to hold all our important paths.
// Note: `pub` makes these fields accessible to other modules (like `main.rs`)
#[derive(Debug)]
pub struct ConfigPaths {
    // pub config_dir: PathBuf,
    pub user_pref_file: PathBuf,
    pub user_wall_dir: PathBuf,
    // pub runtime_dir: PathBuf,
    pub lock_file: PathBuf,
    pub hypr_config_dir: PathBuf,
    pub quickshell_config_dir: PathBuf,
    pub user_capture_dir: PathBuf,
}

/// This is the Rust struct for the /tmp/whisker.lck file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Lockfile {
    pub pid: u32,
}

/// Finds all necessary paths and creates directories if they're missing.
// `pub` makes this function accessible to `main.rs`
pub fn get_config_paths() -> Result<ConfigPaths> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("shelly");

    // Find ~/.config/hypr
    let hypr_config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("hypr");

    // Quickshell looks in ~/.config/quickshell/
    let quickshell_config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("quickshell");
    
    let user_wall_dir = dirs::picture_dir()
        .context("Could not find picture directory")?
        .join("wallpapers");

    let user_capture_dir = dirs::picture_dir() 
        .context("Could not find picture directory")?
        .join("screenshots");
    
    let runtime_dir = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("shelly");

    // Create dirs if they don't exist
    fs::create_dir_all(&config_dir)
        .context(format!("Failed to create config dir at {:?}", config_dir))?;
    fs::create_dir_all(&hypr_config_dir) 
        .context(format!("Failed to create hypr config dir at {:?}", hypr_config_dir))?;
    fs::create_dir_all(&quickshell_config_dir) 
        .context(format!("Failed to create quickshell config dir at {:?}", quickshell_config_dir))?;
    fs::create_dir_all(&user_wall_dir)
        .context(format!("Failed to create wallpaper dir at {:?}", user_wall_dir))?;
    fs::create_dir_all(&user_capture_dir) 
        .context(format!("Failed to create capture dir at {:?}", user_capture_dir))?;
    fs::create_dir_all(&runtime_dir)
        .context(format!("Failed to create runtime dir at {:?}", runtime_dir))?;

    Ok(ConfigPaths {
        user_pref_file: config_dir.join("preferences.json"),
        lock_file: runtime_dir.join("shelly.lck"),
        // config_dir,
        user_wall_dir,
        // runtime_dir,
        hypr_config_dir,
        quickshell_config_dir,
        user_capture_dir,
    })
}