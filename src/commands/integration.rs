use crate::config::ConfigPaths;
use anyhow::{bail, Context, Result};
use std::fs;
use std::process::{Command, Stdio};

const SHELLY_SHELL_REPO: &str = "https://github.com/manpreet113/shelly-shell.git";

pub fn handle_integration(paths: &ConfigPaths) -> Result<()> {
    let target_qml_path = paths.quickshell_config_dir.join("shelly-shell");
    if target_qml_path.exists() {
        println!("shelly-shell already cloned to {:?}", target_qml_path);
    } else {
        println!("Cloning shelly-shell into {:?}", target_qml_path);
        
        let status = Command::new("git")
            .arg("clone")
            .arg(SHELLY_SHELL_REPO)
            .arg(&target_qml_path)
            .stdout(Stdio::inherit()) // Show git's output
            .stderr(Stdio::inherit()) // Show git's errors
            .status()
            .context("Failed to run git. Is it installed?")?;

        if !status.success() {
            bail!("git clone failed. See output above.");
        }
        println!("Successfully cloned shelly-shell.");
    }

    println!(
        "Copying Hyprland configs to: {:?}",
        paths.hypr_config_dir.display()
    );

    let integration_files_dir = target_qml_path.join("integrations");
    let hypr_files = [
        "hypr-execs.conf",
        "hypr-keybinds.conf",
        "hypr-rules.conf",
    ];

    for file_name in hypr_files {
        let source_path = integration_files_dir.join(file_name);
        let dest_path = paths.hypr_config_dir.join(file_name);

        if !source_path.exists() {
            println!(
                "Warning: Source file not found, skipping: {:?}",
                source_path
            );
            continue;
        }

        fs::copy(&source_path, &dest_path).context(format!(
            "Failed to copy {:?} to {:?}",
            source_path, dest_path
        ))?;
        
        println!("  -> Copied {}", dest_path.display());
    }

    println!("\nIntegration complete!");
    println!("Log out and log back in, then run `shelly shell start`");

    Ok(())
}