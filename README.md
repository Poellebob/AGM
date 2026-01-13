# AGM - Advanced Game Mod Manager

A cross-platform mod manager with CLI, TUI, and GUI interfaces.

## Build
```bash
# TUI (default)
cargo build --release

# GUI
cargo build --release --features gui

# Both
cargo build --release --all-features
```

## Usage
```bash
# TUI mode
./target/release/agm

# GUI mode
./target/release/agm --gui

# CLI commands
./target/release/agm add my-mod
./target/release/agm list
```
