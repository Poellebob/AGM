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
}

pub fn load_config(path: String) -> Config {
    let raw_file_string = fs::read_to_string(&path)
        .expect(&format!("Could not read file at {}", &path));
    let config: Config = serde_yaml::from_str(&raw_file_string)
        .expect("Could not parse config file");
    
    config
}
