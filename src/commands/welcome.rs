use crate::config::{ConfigPaths, Lockfile};
use anyhow::{bail, Context, Result};
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use sysinfo::{Pid, System};

fn is_daemon_running(lock: &Lockfile) -> bool {
    let s = System::new_all();
    s.process(Pid::from(lock.pid as usize)).is_some()
}

fn run_detached_welcome(welcome_qml_path: &Path) -> Result<u32> {
    let path_str = welcome_qml_path.to_str()
        .context("Welcome QML path is not valid UTF-8")?;

    let cmd_str = format!(
        "nohup quickshell -p \"{}\" >/dev/null 2>&1 & echo $!",
        path_str
    );

    let mut child = Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd_str)
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run /bin/sh")?;

    let status = child.wait()?;
    if !status.success() {
        bail!("Failed to get PID from detached command");
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

pub fn handle_welcome_start(paths: &ConfigPaths, show_stdout: bool) -> Result<()> {
    if paths.welcome_lock_file.exists() {
        let content = fs::read_to_string(&paths.welcome_lock_file)?;
        let lock: Lockfile = serde_json::from_str(&content)?;
        
        if is_daemon_running(&lock) {
            println!("Welcome screen is already running! (PID: {})", lock.pid);
            return Ok(());
        } else {
            println!("Found stale lockfile. Removing it.");
            fs::remove_file(&paths.welcome_lock_file)?;
        }
    }

    if !paths.welcome_qml_file.exists() {
        bail!(
            "Welcome QML not found at: {:?}\nDid you run `shelly integration` first?",
            paths.welcome_qml_file
        );
    }
    let qml_path_str = paths.welcome_qml_file.to_str().unwrap();

    if show_stdout {
        println!("Starting welcome screen in foreground...");
        let mut child = Command::new("quickshell")
            .args(["-p", qml_path_str])
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to start quickshell. Is it installed?")?;

        println!("Welcome screen running! (PID: {})", child.id());

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }
        child.wait()?;
        println!("Welcome screen exited.");
    } else {
        println!("Starting welcome screen in background...");
        let pid = run_detached_welcome(&paths.welcome_qml_file)?;

        let lock = Lockfile { pid };
        let lock_content = serde_json::to_string_pretty(&lock)?;
        fs::write(&paths.welcome_lock_file, lock_content)?;

        println!("Welcome screen running! (PID: {})", pid);
    }
    
    Ok(())
}

pub fn handle_welcome_stop(paths: &ConfigPaths) -> Result<()> {
    if !paths.welcome_lock_file.exists() {
        println!("Welcome screen is not running.");
        return Ok(());
    }
    
    let content = fs::read_to_string(&paths.welcome_lock_file)?;
    let lock: Lockfile = serde_json::from_str(&content)?;

    if !is_daemon_running(&lock) {
        println!("Welcome screen is not running (found stale lockfile).");
        fs::remove_file(&paths.welcome_lock_file)?;
        return Ok(());
    }
    
    let s = System::new_all();
    if let Some(process) = s.process(Pid::from(lock.pid as usize)) {
        println!("Stopping welcome screen (PID: {})...", lock.pid);
        process.kill();
    } else {
        println!("Process {} not found, but lockfile exists.", lock.pid);
    }
    
    fs::remove_file(&paths.welcome_lock_file)?;
    println!("Welcome screen stopped!");
    Ok(())
}