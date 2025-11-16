use anyhow::{Context, Result};
use std::process::{Command, Stdio};

/// This is a simple passthrough to the `qs` command,
/// exactly like the original IpcCommand.
pub fn handle_ipc(args: &[String]) -> Result<()> {
    // Build the full command: "qs -c shelly-shell ipc call arg1 arg2 ..."
    let mut cmd = Command::new("qs");
    cmd.arg("-c")
        .arg("shelly-shell")
        .arg("ipc")
        .arg("call")
        .args(args); // Add all the user's arguments

    // We don't want to wait for this. We want to fire-and-forget,
    // just like the original `Utils.runDetached`.
    // Spawning without `status()` or `wait()` does exactly that.
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    
    cmd.spawn()
        .context("Failed to run 'qs -c shelly-shell ...'. Is quickshell installed?")?;

    Ok(())
}