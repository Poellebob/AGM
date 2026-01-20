# AGM - A Generic Mod-manager 

A cross-platform mod manager with CLI, TUI, and GUI interfaces.
AGM is in very early development and does not have basic features yet.

## Features

### Configuration
- `agm config --nexus-api-key <key>`: Sets your Nexus Mods API key.
- `agm config --editor <command>`: Sets your preferred text editor (e.g., `nano`, `vim`, `code`). The `EDITOR` environment variable is used as a fallback.

### Profile Management
- `agm profile list`: Lists all available profiles.
- `agm profile add <game_name> [--name <profile_name>]`: Creates a new game profile. You'll be prompted for the game's installation path, and the new profile will open in your configured editor.
- `agm profile edit <profile_name>`: Opens an existing profile in your editor.
- `agm profile remove <profile_name>`: Deletes a profile and its configuration file.

### Mod Installation
- `agm install <path_to_zip> --profile <profile_name>`: Installs a mod from a `.zip` file.
  - Unpacks the mod into AGM's central storage directory.
  - Automatically guesses file placements based on `mime` types defined in the profile.
  - Interactively prompts for placement for any files that could not be automatically placed.
  - Creates symlinks from the game's directory to the mod files in storage, enabling instant activation.

## Usage

### Profile
- `agm profile list`
- `agm profile add <game_name> [--name <profile_name>]`
- `agm profile edit <profile_name>`
- `agm profile remove <profile_name>`

### Preset
- `agm preset switch <game> <preset>`
- `agm preset list [--profile <game>]`
- `agm preset add <game> <name> #list of urls or archives`
- `agm preset edit <game> <name>`
- `agm preset remove <game> <preset>`
- `agm preset remove <game> -a #--all`
- `agm preset disable <game>`

### Config
- `agm config --nexus-api-key <key>`
- `agm config --editor <command>`

### Install
- `agm install <path_to_zip> --profile <profile_name>`

## Profiles | Game spec

Profiles are yaml files that define what the games mod stukture looks like.

```yaml
game:
  name: ExampleGame
  path: game/  #path to game dir

layout:
- name: bin
  type: dir
  sub:
  - name: binmod
    type: moddir
    mime: [zip, pkg] # .zip or pkg's go here

- name: scriptsmod
  type: moddir
  mime: [src, gam]
```

## Preset | Mod collection spec (mod pack)

```yaml
game: <name of game>

mods:
- some mod
- name: other mod
```

might add version controll that is why you can do `name: mod`

## Mod spec

```yaml
name: cool mod
url: <url>
files:
- target: <dir/>
  point:  "@<name>" # from the profiles section eg. scriptmod or binmod

- target: <dir/>
  point: <dir/> #with the root being the games dir

- target: <file>
  point: <dir/>
```


## Storage


```tree
~/
├── data-dir/
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
│
└── config-dir/
    └── agm.yaml
```


## Config

This normaly should not be touched

```yaml
profile:
- game1
- game2
preset:
- game: game1
  alias:
  - G1
  - Game1
  presets:
    modpak1
    modpak2
- game: game2
  alias:
  - Ga2
  presets:
    modpak1
```


