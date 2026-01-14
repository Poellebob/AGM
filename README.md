# AGM - A Generic Mod-manager 

A cross-platform mod manager with CLI, TUI, and GUI interfaces.
AGM is in very early development and does not have basic features yet.

## Planned features
- [ ] Game profiles
- [ ] Add, remove, or disable mods
- [ ] Game presets
- [ ] Exporting presets
- [ ] Importing presets
- [ ] Downloading presets from URL or Git
- [ ] Cli
- [ ] Tui
- [ ] Gui

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
