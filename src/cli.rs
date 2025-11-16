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
    Wallpaper {
        path: PathBuf,

        #[clap(long)]
        no_scheme_gen: bool,
    },

    Prefs {
        #[command(subcommand)]
        action: PrefsAction,
    },

    List {
        #[command(subcommand)]
        list_type: ListType,
    },

    Shell {
        #[command(subcommand)]
        action: ShellAction,
    },

    Notify {
        title: String,
        body: String,
    },

    Integration,

    #[command(allow_external_subcommands = true, trailing_var_arg = true)]
    Ipc {
        #[arg(required = true, num_args = 1..)]
        args: Vec<String>,
    },

    Screen {
        #[clap(long)]
        region: bool,

        #[clap(long)]
        copy: bool,
    },

    Welcome {
        #[command(subcommand)]
        action: WelcomeAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum PrefsAction {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ListType {
    Wallpapers,
}

#[derive(Subcommand, Debug)]
pub enum ShellAction {
    Start {
        #[clap(long)]
        stdout: bool,
    },
    Stop,
    Status,
}

#[derive(Subcommand, Debug)]
pub enum WelcomeAction {
    Start {
        #[clap(long)]
        stdout: bool,
    },
    Stop,
}