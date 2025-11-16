use crate::config::{ConfigPaths, Lockfile};
use anyhow::{Context, Result};
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use sysinfo::{Pid, System};

fn run_detached_quickshell() -> Result<u32> {
    let cmd_str = "nohup qs -c shelly-shell >/dev/null 2>&1 & echo $!";
    
    let mut child = Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd_str)
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run /bin/sh")?;

    let status = child.wait()?;
    if !status.success() {
        anyhow::bail!("Failed to get PID from detached command");
    }

    let mut stdout_str = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut stdout_str)?;
    }

    let pid = stdout_str
        .trim()
        .parse::<u32>()
        .context("Failed to parse PID from stdout")?;

    Ok(pid)
}

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

    if show_stdout {
        println!("Starting shelly shell in foreground...");
        let mut child = Command::new("qs")
            .args(["-c", "shelly-shell"])
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to start quickshell. Is it installed?")?;

        println!("shelly shell successfully running! (PID: {})", child.id());

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }
        child.wait()?;
        println!("shelly shell exited.");
    } else {
        println!("Starting shelly shell in background...");
        let pid = run_detached_quickshell()?;

        let lock = Lockfile { pid };
        let lock_content = serde_json::to_string_pretty(&lock)?;
        fs::write(&paths.lock_file, lock_content)?;

        println!("shelly shell successfully running! (PID: {})", pid);
    }
    
    Ok(())
}

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

fn is_daemon_running(lock: &Lockfile) -> bool {
    let s = System::new_all();
    s.process(Pid::from(lock.pid as usize)).is_some()
}