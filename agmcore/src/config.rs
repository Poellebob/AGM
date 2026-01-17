use std::fs;
use std::path::Path;
use dirs_next::{config_dir, data_dir};
use std::io;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub profile: Vec<String>,
    pub preset: Vec<Preset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Preset {
    pub name: String,
    pub mods: Vec<String>,
}

pub fn ensure_config_file() {
    if let Some(mut program_dir) = config_dir() {
        program_dir.push("AGM");

        if !program_dir.exists() {
            fs::create_dir_all(&program_dir)
                .expect(&format!("Could not create dir at {}", &program_dir.display()));
            println!("Created program directory: {}", program_dir.display());
        } else {
            println!("Program directory already exists: {}", program_dir.display());
        }

        let config_file = program_dir.join("config.yaml");

        if !config_file.exists() {
            fs::File::create(&config_file)
                .expect(&format!("Could not create file at {}", &config_file.display()));
            println!("Created config file: {}", config_file.display());
        } else {
            println!("Config file already exists: {}", config_file.display());
        }
    }

    if let Some(mut data_dir) = data_dir() {
        data_dir.push("AGM");

        if !data_dir.exists() {
            fs::create_dir_all(&program_dir)
                .expect(&format!("Could not create dir at {}", &data_dir.display()));
            println!("Created program directory: {}", data_dir.display());
        } else {
            println!("Program directory already exists: {}", data_dir.display());
        }
    }
}

pub fn load_config() -> Config {
    let base = data_dir()
        .expect("Failed to resolve data directory")
        .join("agm");

    let profiles_dir = base.join("profiles");
    let presets_dir = base.join("presets");

    let mut profiles: Vec<String> = Vec::new();
    let mut presets: Vec<Preset> = Vec::new();

    // -----------------------------
    // Load profiles/*.yaml
    // -----------------------------
    if profiles_dir.exists() {
        for entry in fs::read_dir(&profiles_dir)
            .expect("Failed to read profiles directory")
        {
            let path = entry
                .expect("Failed to read profiles directory entry")
                .path();

            if is_yaml(&path) {
                let text = fs::read_to_string(&path)
                    .expect("Failed to read profile file");

                let mut list: Vec<String> = serde_yaml::from_str(&text)
                    .expect("Failed to parse profile yaml");

                profiles.append(&mut list);
            }
        }
    }

    // -----------------------------
    // Load presets/<game>/*.yaml
    // -----------------------------
    if presets_dir.exists() {
        for game_dir in fs::read_dir(&presets_dir)
            .expect("Failed to read presets directory")
        {
            let game_path = game_dir
                .expect("Failed to read preset game directory entry")
                .path();

            if !game_path.is_dir() {
                continue;
            }

            for preset_file in fs::read_dir(&game_path)
                .expect("Failed to read game preset directory")
            {
                let path = preset_file
                    .expect("Failed to read preset file entry")
                    .path();

                if is_yaml(&path) {
                    let text = fs::read_to_string(&path)
                        .expect("Failed to read preset file");

                    let preset: Preset = serde_yaml::from_str(&text)
                        .expect("Failed to parse preset yaml");

                    presets.push(preset);
                }
            }
        }
    }

    Config {
        profile: profiles,
        preset: presets,
    }
}

fn is_yaml(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("yaml") | Some("yml")
    )
}

/*pub fn load_config(path: String) -> Config {
    let raw_file_string = fs::read_to_string(&path)
        .expect(&format!("Could not read file at {}", &path));
    let config: Config = serde_yaml::from_str(&raw_file_string)
        .expect("Could not parse config file");
    
    config
}*/
