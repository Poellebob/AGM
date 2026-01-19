use dirs_next::{config_dir, data_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub profiles: Vec<String>,
    #[serde(default)]
    pub presets: Vec<String>,
    #[serde(default)]
    pub nexus_api_key: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            presets: Vec::new(),
            nexus_api_key: None,
        }
    }

    pub fn get_config_dir() -> PathBuf {
        let mut program_dir = config_dir().unwrap();
        program_dir.push("AGM");
        program_dir
    }

    pub fn get_data_dir() -> PathBuf {
        let mut program_dir = data_dir().unwrap();
        program_dir.push("AGM");
        program_dir
    }

    pub fn ensure_config_dirs() {
        let config_dir = Self::get_config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).expect(&format!(
                "Could not create dir at {}",
                &config_dir.display()
            ));
        }

        let data_dir = Self::get_data_dir();
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)
                .expect(&format!("Could not create dir at {}", &data_dir.display()));
        }
    }

    pub fn load() -> Self {
        let config_dir = Self::get_config_dir();
        let config_file = config_dir.join("config.yaml");

        if !config_file.exists() {
            return Self::new();
        }

        let raw_file_string = fs::read_to_string(&config_file).expect(&format!(
            "Could not read file at {}",
            &config_file.display()
        ));
        match serde_yaml::from_str(&raw_file_string) {
            Ok(config) => config,
            Err(e) => {
                eprintln!(
                    "Warning: Could not parse config file at {}: {}. Using default config.",
                    &config_file.display(),
                    e
                );
                Self::new()
            }
        }
    }

    pub fn get_socket_path() -> PathBuf {
        let mut data_dir = Self::get_data_dir();
        data_dir.push("agm.sock");
        data_dir
    }

    pub fn save(&self) {
        let config_dir = Self::get_config_dir();
        let config_file = config_dir.join("config.yaml");

        let yaml_string = serde_yaml::to_string(self).unwrap();
        fs::write(config_file, yaml_string).unwrap();
    }
}
