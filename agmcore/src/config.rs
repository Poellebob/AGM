use std::fs;
use dirs_next::config_dir;
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

pub fn ensure_config_file() -> io::Result<()> {
    if let Some(mut program_dir) = config_dir() {
        program_dir.push("AGM");

        if !program_dir.exists() {
            fs::create_dir_all(&program_dir)?;
            println!("Created program directory: {}", program_dir.display());
        } else {
            println!("Program directory already exists: {}", program_dir.display());
        }

        let config_file = program_dir.join("config.yaml");

        if !config_file.exists() {
            fs::File::create(&config_file)?;
            println!("Created config file: {}", config_file.display());
        } else {
            println!("Config file already exists: {}", config_file.display());
        }
    }

    Ok(())
}

pub fn load_config(path: String) {
    let rawFileString = fs::read_to_string(path)
        .expect("Could not read file at {}", path);
    let config: Config = serde_yaml::from_str(rawFileString);

    Ok(config)
}
