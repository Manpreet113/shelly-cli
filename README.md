# Shelly

A helper CLI tool for managing wallpapers, preferences, and shell daemon operations.

## Features

- **Wallpaper Management**: Change wallpapers and optionally generate color schemes
- **Preferences**: Get and set user preferences using a key-value store
- **List Operations**: List available wallpapers and other resources
- **Shell Daemon**: Start, stop, and check the status of the shelly daemon
- **Integration**: Automatically clone and set up shelly-shell QML components and Hyprland configurations
- **Notifications**: Send desktop notifications from the command line
- **Screen Capture**: Take screenshots with region selection and clipboard support
- **Welcome Screen**: Launch and manage the shelly welcome screen
- **IPC**: Send IPC messages to the running shelly-shell

## Installation

### From Source

```bash
git clone https://github.com/manpreet113/shelly.git
cd shelly
cargo build --release
```

The binary will be available at `target/release/shelly`.

### Installing Globally

```bash
cargo install --path .
```

## Usage

```bash
shelly <COMMAND>
```

### Commands

#### Wallpaper

Change the wallpaper and optionally generate a new color scheme:

```bash
shelly wallpaper /path/to/image.jpg

shelly wallpaper /path/to/image.jpg --no-scheme-gen
```

Supports both images and videos. For videos, a frame is extracted for color generation.

#### Preferences

Get or set user preferences:

```bash
shelly prefs get <KEY>
shelly prefs get theme.dark

shelly prefs set <KEY> <VALUE>
shelly prefs set theme.dark true
```

#### List

List available resources:

```bash
shelly list wallpapers
```

#### Shell Daemon

Manage the shelly daemon:

```bash
shelly shell start

shelly shell start --stdout

shelly shell stop

shelly shell status
```

#### Integration

Set up the shelly-shell environment:

```bash
shelly integration
```

This command will:

- Clone the shelly-shell repository to your QuickShell config directory
- Copy Hyprland configuration files (execs, keybinds, rules) to your Hyprland config directory
- Set up the necessary integrations for the shell to work properly

#### Notify

Send desktop notifications:

```bash
shelly notify "Title" "Body message"
```

#### Screen Capture

Take screenshots:

```bash
shelly screen

shelly screen --region

shelly screen --region --copy
```

#### Welcome Screen

Manage the welcome screen:

```bash
shelly welcome start

shelly welcome start --stdout

shelly welcome stop
```

#### IPC

Send IPC messages to shelly-shell:

```bash
shelly ipc <ARGS>
```

## Configuration

Shelly uses platform-specific configuration directories:

- **Linux**: `~/.config/shelly/`
- **macOS**: `~/Library/Application Support/shelly/`
- **Windows**: `%APPDATA%\shelly\`

## Dependencies

- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [serde](https://github.com/serde-rs/serde) & [serde_json](https://github.com/serde-rs/json) - Serialization/deserialization
- [dirs](https://github.com/dirs-dev/dirs-rs) - Cross-platform config directory detection
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Process management
- [notify-rust](https://github.com/hoodie/notify-rust) - Desktop notifications
- [chrono](https://github.com/chronotope/chrono) - Date and time

## External Dependencies

- **matugen** - For color scheme generation
- **ffmpeg** - For video frame extraction
- **quickshell (qs)** - For running the shell
- **grim** - For screenshots
- **slurp** - For region selection
- **wl-copy** - For clipboard operations

## Development

### Project Structure

```text
shelly/
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── config.rs
│   └── commands/
│       ├── mod.rs
│       ├── wallpaper.rs
│       ├── prefs.rs
│       ├── list.rs
│       ├── shell.rs
│       ├── integration.rs
│       ├── notify.rs
│       ├── screen.rs
│       ├── ipc.rs
│       └── welcome.rs
├── integrations/
├── Cargo.toml
└── README.md
```

### Building

```bash
cargo build
```

### Running

```bash
cargo run -- <COMMAND>
```

### Testing

```bash
cargo test
```

## License

See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
