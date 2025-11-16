use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "shelly", version = "0.1.0", about = "A helper CLI, inspired by whisker-cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Change the wallpaper (replaces WallpapersCommand.hx)
    Wallpaper {
        /// The path to the wallpaper file
        path: PathBuf,

        /// Skip generating a new color scheme
        #[clap(long)]
        no_scheme_gen: bool,
    },

    /// Get or set user preferences (replaces PreferencesCommand.hx)
    Prefs {
        #[command(subcommand)]
        action: PrefsAction,
    },

    /// Get a list of something (replaces ListsCommand.hx)
    List {
        #[command(subcommand)]
        list_type: ListType,
    },

    /// Start, stop, or check the shell daemon (replaces ShellCommand.hx)
    Shell {
        #[command(subcommand)]
        action: ShellAction,
    },

    /// Send a desktop notification (replaces NotifyCommand.hx)
    Notify {
        /// The notification title
        title: String,
        /// The notification body
        body: String,
    },
    /// Integrate shelly with Hyprland
    Integration,
}

#[derive(Subcommand, Debug)]
pub enum PrefsAction {
    /// Get a preference value
    Get {
        /// The key to get (e.g., "theme.dark")
        key: String,
    },
    /// Set a preference value
    Set {
        /// The key to set (e.g., "theme.dark")
        key: String,
        /// The value to set
        value: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ListType {
    /// List available wallpapers
    Wallpapers,
}

#[derive(Subcommand, Debug)]
pub enum ShellAction {
    /// Start the shelly daemon
    Start {
        /// Print daemon stdout to the console
        #[clap(long)]
        stdout: bool,
    },
    /// Stop the shelly daemon
    Stop,
    /// Check the status of the shelly daemon
    Status,
}