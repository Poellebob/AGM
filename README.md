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

### Profile

- `agm profile list`
- `agm profile add <game>`
- `agm profile edit <game>`

### Preset

- `agm preset switch <game> <preset>`
- `agm preset list #<game>`
- `agm preset add <game> #list of urls or archives`
- `agm preset edit <game> <preset>`
- `agm preset delete <game> <preset>`
- `agm preset delete <game> -a #--all`
- `agm preset disable <game>`


## Profiles

Profiles are yaml files that define what the games mod stukture looks like.

```yaml

game:
  name: ExampleGame
  path: game/

layout:
  bin:
    mods:
      moddir: true

  script:
    moddir: true

```

## Preset

```yaml

game: <name of game>

mods:
- name: some mod
  url: <url>
  files:
  - target: <dir/>
    point: <dir/>

  - target: <file>
    point: <dir/>

- name: other mod
  url: <url>
  files:
  - target: <file>
    point: <dir/>

```


doc["mods"][mod_index]["files"][file_index]["target"]