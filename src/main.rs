mod cli;
mod config;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, PrefsAction, ListType, ShellAction, WelcomeAction};
use config::get_config_paths;

use commands::{
    list::handle_list_wallpapers,
    prefs::{handle_prefs_get, handle_prefs_set},
    shell::{handle_shell_start, handle_shell_stop, handle_shell_status},
    wallpaper::handle_wallpaper,
    notify::handle_notify,
    integration::handle_integration,
    ipc::handle_ipc,
    screen::handle_screen,
    welcome::{handle_welcome_start, handle_welcome_stop},
};

fn main() -> Result<()> {
    let paths = get_config_paths()?;
    let cli = Cli::parse();

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
        },
        Commands::Ipc { args } => {
            handle_ipc(&args)?;
        },
        Commands::Screen { region, copy } => {
            handle_screen(&paths, region, copy)?;
        },
        Commands::Welcome { action } => match action {
            WelcomeAction::Start { stdout } => handle_welcome_start(&paths, stdout)?,
            WelcomeAction::Stop => handle_welcome_stop(&paths)?,
        },
    }

    Ok(())
}