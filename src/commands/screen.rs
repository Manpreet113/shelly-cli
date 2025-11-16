use crate::config::ConfigPaths;
use anyhow::{bail, Context, Result};
use chrono::Local;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

/// Get a region from slurp
fn get_slurp_region() -> Result<String> {
    let output = Command::new("slurp")
        .output()
        .context("Failed to run slurp. Is it installed?")?;

    if !output.status.success() {
        bail!(
            "slurp failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Copy image data to wl-copy
fn copy_to_clipboard(image_data: &[u8]) -> Result<()> {
    let mut child = Command::new("wl-copy")
        .arg("--type")
        .arg("image/png")
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to run wl-copy. Is it installed?")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(image_data)
            .context("Failed to pipe image data to wl-copy")?;
    }

    let status = child.wait()?;
    if !status.success() {
        bail!("wl-copy command failed");
    }

    Ok(())
}

// `pub` so `main.rs` can call it.
pub fn handle_screen(paths: &ConfigPaths, region: bool, copy: bool) -> Result<()> {
    // 1. Get the region string *before* calling grim
    let region_str = if region {
        println!("Please select a region...");
        Some(get_slurp_region()?)
    } else {
        None
    };

    // 2. Generate the timestamp and output path
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let output_path = paths
        .user_capture_dir
        .join(format!("screenshot_{}.png", timestamp));
    let output_str = output_path
        .to_str()
        .context("Failed to create output path string")?;

    // 3. Build the grim command
    let mut cmd = Command::new("grim");
    if let Some(geom) = &region_str {
        cmd.arg("-g").arg(geom);
    }
    
    // We are piping to stdout, not saving to a file *yet*
    cmd.stdout(Stdio::piped());
    
    // If not copying, we save directly to the file
    if !copy {
        cmd.arg(output_str);
    }

    // 4. Run grim
    let output = cmd
        .output()
        .context("Failed to run grim. Is it installed?")?;

    if !output.status.success() {
        bail!("grim failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // 5. Handle copy or save
    if copy {
        let image_data = &output.stdout;
        
        // Copy to clipboard
        copy_to_clipboard(image_data)?;

        // *Also* save the file
        fs::write(&output_path, image_data)
            .context("Failed to save screenshot file after copying")?;
        
        println!("{} (copied to clipboard)", output_str);
    } else {
        // File was already saved by grim
        println!("{}", output_str);
    }

    Ok(())
}