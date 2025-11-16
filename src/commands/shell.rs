use crate::config::{ConfigPaths, Lockfile};
use anyhow::{Context, Result};
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use sysinfo::{Pid, System};

// `pub` so `main.rs` can call it.
pub fn handle_shell_start(paths: &ConfigPaths, show_stdout: bool) -> Result<()> {
    if paths.lock_file.exists() {
        let content = fs::read_to_string(&paths.lock_file)?;
        let lock: Lockfile = serde_json::from_str(&content)?;
        
        if is_daemon_running(&lock) {
            println!("shelly shell is already running! (PID: {})", lock.pid);
            return Ok(());
        } else {
            println!("Found stale lockfile. Removing it.");
            fs::remove_file(&paths.lock_file)?;
        }
    }

    println!("Starting shelly shell...");
    let mut command = Command::new("sleep");
    command.arg("600"); // 10 minute placeholder daemon
    
    if !show_stdout {
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
    } else {
        command.stdout(Stdio::piped());
    }

    let mut child = command
        .spawn()
        .context("Failed to start shelly shell daemon")?;

    let pid = child.id();
    
    let lock = Lockfile { pid };
    let lock_content = serde_json::to_string_pretty(&lock)?;
    fs::write(&paths.lock_file, lock_content)?;

    println!("shelly shell successfully running! (PID: {})", pid);

    if show_stdout {
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }
        child.wait()?;
        println!("shelly shell exited.");
        fs::remove_file(&paths.lock_file)?;
    }
    
    Ok(())
}

// `pub` so `main.rs` can call it.
pub fn handle_shell_stop(paths: &ConfigPaths) -> Result<()> {
    if !paths.lock_file.exists() {
        println!("shelly shell is not running.");
        return Ok(());
    }
    
    let content = fs::read_to_string(&paths.lock_file)?;
    let lock: Lockfile = serde_json::from_str(&content)?;

    if !is_daemon_running(&lock) {
        println!("shelly shell is not running (found stale lockfile).");
        fs::remove_file(&paths.lock_file)?;
        return Ok(());
    }
    
    let s = System::new_all();
    if let Some(process) = s.process(Pid::from(lock.pid as usize)) {
        println!("Stopping shelly shell (PID: {})...", lock.pid);
        process.kill();
    } else {
        println!("Process {} not found, but lockfile exists.", lock.pid);
    }
    
    fs::remove_file(&paths.lock_file)?;
    println!("shelly shell stopped!");
    Ok(())
}

// `pub` so `main.rs` can call it.
pub fn handle_shell_status(paths: &ConfigPaths) -> Result<()> {
    if !paths.lock_file.exists() {
        println!("shelly shell is NOT running.");
        return Ok(());
    }
    
    let content = fs::read_to_string(&paths.lock_file)?;
    let lock: Lockfile = serde_json::from_str(&content)?;
    
    if is_daemon_running(&lock) {
        println!("shelly shell IS running (PID: {}).", lock.pid);
    } else {
        println!("shelly shell is NOT running (found stale lockfile).");
        fs::remove_file(&paths.lock_file)?;
    }
    Ok(())
}

// --- Helper Function ---
// (NOT `pub`, private to this module)

fn is_daemon_running(lock: &Lockfile) -> bool {
    let s = System::new_all();
    s.process(Pid::from(lock.pid as usize)).is_some()
}