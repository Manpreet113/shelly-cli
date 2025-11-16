use anyhow::{Context, Result};
use std::process::{Command, Stdio};

pub fn handle_ipc(args: &[String]) -> Result<()> {
    let mut cmd = Command::new("qs");
    cmd.arg("-c")
        .arg("shelly-shell")
        .arg("ipc")
        .arg("call")
        .args(args);

    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    
    cmd.spawn()
        .context("Failed to run 'qs -c shelly-shell ...'. Is quickshell installed?")?;

    Ok(())
}