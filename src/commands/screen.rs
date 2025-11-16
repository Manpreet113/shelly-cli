use crate::config::ConfigPaths;
use anyhow::{bail, Context, Result};
use chrono::Local;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

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

pub fn handle_screen(paths: &ConfigPaths, region: bool, copy: bool) -> Result<()> {
    let region_str = if region {
        println!("Please select a region...");
        Some(get_slurp_region()?)
    } else {
        None
    };

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let output_path = paths
        .user_capture_dir
        .join(format!("screenshot_{}.png", timestamp));
    let output_str = output_path
        .to_str()
        .context("Failed to create output path string")?;

    let mut cmd = Command::new("grim");
    if let Some(geom) = &region_str {
        cmd.arg("-g").arg(geom);
    }

    cmd.stdout(Stdio::piped());

    if !copy {
        cmd.arg(output_str);
    }

    let output = cmd
        .output()
        .context("Failed to run grim. Is it installed?")?;

    if !output.status.success() {
        bail!("grim failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    if copy {
        let image_data = &output.stdout;

        copy_to_clipboard(image_data)?;

        fs::write(&output_path, image_data)
            .context("Failed to save screenshot file after copying")?;

        println!("{} (copied to clipboard)", output_str);
    } else {
        println!("{}", output_str);
    }

    Ok(())
}