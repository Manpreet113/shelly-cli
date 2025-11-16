use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ConfigPaths {
    pub user_pref_file: PathBuf,
    pub user_wall_dir: PathBuf,
    pub lock_file: PathBuf,
    pub hypr_config_dir: PathBuf,
    pub user_capture_dir: PathBuf,
    pub welcome_lock_file: PathBuf,
    pub welcome_qml_file: PathBuf,
    pub shell_qml_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lockfile {
    pub pid: u32,
}

pub fn get_config_paths() -> Result<ConfigPaths> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("shelly");

    let hypr_config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("hypr");

    let shell_qml_dir = PathBuf::from("/usr/share/quickshell/shelly-shell");

    let welcome_qml_file = shell_qml_dir.join("welcome.qml");

    let user_wall_dir = dirs::picture_dir()
        .context("Could not find picture directory")?
        .join("wallpapers");

    let user_capture_dir = dirs::picture_dir()
        .context("Could not find picture directory")?
        .join("screenshots");

    let runtime_dir = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("shelly");

    let welcome_lock_file = runtime_dir.join("shelly-welcome.lck");

    fs::create_dir_all(&config_dir)
        .context(format!("Failed to create config dir at {:?}", config_dir))?;
    fs::create_dir_all(&hypr_config_dir)
        .context(format!("Failed to create hypr config dir at {:?}", hypr_config_dir))?;
    fs::create_dir_all(&user_wall_dir)
        .context(format!("Failed to create wallpaper dir at {:?}", user_wall_dir))?;
    fs::create_dir_all(&user_capture_dir)
        .context(format!("Failed to create capture dir at {:?}", user_capture_dir))?;
    fs::create_dir_all(&runtime_dir)
        .context(format!("Failed to create runtime dir at {:?}", runtime_dir))?;

    Ok(ConfigPaths {
        user_pref_file: config_dir.join("preferences.json"),
        lock_file: runtime_dir.join("shelly.lck"),
        user_wall_dir,
        hypr_config_dir,
        user_capture_dir,
        welcome_lock_file,
        welcome_qml_file,
        shell_qml_dir,
    })
}