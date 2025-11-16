# Shelly

A helper CLI tool for managing wallpapers, preferences, and shell daemon operations.

## Features

- **Wallpaper Management**: Change wallpapers and optionally generate color schemes
- **Preferences**: Get and set user preferences using a key-value store
- **List Operations**: List available wallpapers and other resources
- **Shell Daemon**: Start, stop, and check the status of the shelly daemon

## Installation

### From Source

```bash
git clone <repository-url>
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
# Set a wallpaper (generates color scheme by default)
shelly wallpaper /path/to/image.jpg

# Set a wallpaper without generating a color scheme
shelly wallpaper /path/to/image.jpg --no-scheme-gen
```

#### Preferences

Get or set user preferences:

```bash
# Get a preference value
shelly prefs get <KEY>
shelly prefs get theme.dark

# Set a preference value
shelly prefs set <KEY> <VALUE>
shelly prefs set theme.dark true
```

#### List

List available resources:

```bash
# List available wallpapers
shelly list wallpapers
```

#### Shell Daemon

Manage the shelly daemon:

```bash
# Start the daemon
shelly shell start

# Start the daemon with stdout output
shelly shell start --stdout

# Stop the daemon
shelly shell stop

# Check daemon status
shelly shell status
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

## Development

### Project Structure

```text
shelly/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI argument definitions
│   ├── config.rs         # Configuration handling
│   └── commands/         # Command implementations
│       ├── mod.rs
│       ├── wallpaper.rs
│       ├── prefs.rs
│       ├── list.rs
│       └── shell.rs
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
