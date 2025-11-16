use crate::config::ConfigPaths;
use anyhow::{Context, Result};
use std::fs;

pub fn handle_integration(paths: &ConfigPaths) -> Result<()> {
    let integration_files_dir = paths.shell_qml_dir.join("integrations");

    if !integration_files_dir.exists() {
        anyhow::bail!(
            "Source directory not found: {:?}\nIs `shelly-shell-git` package installed correctly?",
            integration_files_dir
        );
    }

    println!(
        "Copying Hyprland configs to: {:?}",
        paths.hypr_config_dir.display()
    );

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