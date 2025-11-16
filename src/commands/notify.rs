use anyhow::{Context, Result};
use notify_rust::Notification;

// `pub` so `main.rs` can call it.
// We don't even need the `paths` for this one.
pub fn handle_notify(title: &str, body: &str) -> Result<()> {
    Notification::new()
        .summary(title)
        .body(body)
        .appname("shelly")
        // We could add an icon here if we had one
        .icon("$HOME/.config/quickshell/shelly/logo.png")
        .show()
        .context("Failed to send notification")?;

    Ok(())
}