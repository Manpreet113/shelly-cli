mod cli;
mod config;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, PrefsAction, ListType, ShellAction};
use config::get_config_paths;

// We pull in the *public* handler functions from our new command modules.
use commands::{
    list::handle_list_wallpapers,
    prefs::{handle_prefs_get, handle_prefs_set},
    shell::{handle_shell_start, handle_shell_stop, handle_shell_status},
    wallpaper::handle_wallpaper,
    notify::handle_notify,
    integration::handle_integration,
};

fn main() -> Result<()> {
    // 1. Get paths (from config module)
    let paths = get_config_paths()?;
    
    // 2. Parse CLI (from cli module)
    let cli = Cli::parse();

    // 3. Match and delegate (to commands modules)
    match cli.command {
        Commands::Wallpaper { path, no_scheme_gen } => {
            handle_wallpaper(&paths, &path, no_scheme_gen)?;
        }
        Commands::Prefs { action } => match action {
            PrefsAction::Get { key } => handle_prefs_get(&paths, &key)?,
            PrefsAction::Set { key, value } => handle_prefs_set(&paths, &key, &value)?,
        },
        Commands::List { list_type } => match list_type {
            ListType::Wallpapers => handle_list_wallpapers(&paths)?,
        },
        Commands::Shell { action } => match action {
            ShellAction::Start { stdout } => handle_shell_start(&paths, stdout)?,
            ShellAction::Stop => handle_shell_stop(&paths)?,
            ShellAction::Status => handle_shell_status(&paths)?,
        },
        Commands::Notify { title, body } => {
            handle_notify(&title, &body)?;
        },
        Commands::Integration => {
            handle_integration(&paths)?;
        }
    }

    Ok(())
}