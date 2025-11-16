use anyhow::{Context, Result};
use notify_rust::Notification;

pub fn handle_notify(title: &str, body: &str) -> Result<()> {
    Notification::new()
        .summary(title)
        .body(body)
        .appname("shelly")
        .icon("$HOME/.config/quickshell/shelly/logo.png")
        .show()
        .context("Failed to send notification")?;

    Ok(())
}