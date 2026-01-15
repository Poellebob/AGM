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
- `agm preset add <game> <name> #list of urls or archives`
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
- name: bin
  type: dir
  sub:
  - name: binmod
    type: moddir

- name: scriptsmod
  type: moddir
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
  - target: <dir/>
    point: <name> # from the profiles section eg. scriptmod or binmod
```


## Storage


```tree
~/
├── .local/share/agm/
│   ├── storage/
│   │   ├── game1/
│   │   │   ├── modfile
│   │   │   └── moddir/
│   │   └── game2/
│   │       ├── modfile
│   │       └── modfile
│   ├── profiles/
│   │   ├── game1.yaml
│   │   └── game2.yaml
│   └── presets/
│       ├── game1/
│       │   ├── preset1.yaml
│       │   └── preset2.yaml
│       └── game2/
│           └── preset1.yaml
└── .config/
    └── agm.yaml
```


## Config

This normaly should not be touched

```yaml
profile:
  minecraft
  game1
  game2
preset:
- game: minecraft
  presets:
    create
    pp
- game: game1
  presets:
    modpak1
    modpak2
    pp
- game: game2
  presets:
    modpak1
    pp
```
