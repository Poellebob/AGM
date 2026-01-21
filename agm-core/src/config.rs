use dirs_next::{config_dir, data_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Could not determine home directory")]
    NoHomeDir,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetConfig {
    pub game: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub presets: Vec<String>,
    #[serde(default)]
    pub active_preset: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub profiles: Vec<String>,
    #[serde(default)]
    pub presets: Vec<PresetConfig>,
    #[serde(default)]
    pub nexus_api_key: Option<String>,
    #[serde(default)]
    pub editor: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            presets: Vec::new(),
            nexus_api_key: None,
            editor: None,
        }
    }

    pub fn get_config_dir() -> Result<PathBuf, io::Error> {
        let mut program_dir = config_dir().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine config directory",
            )
        })?;
        program_dir.push("AGM");
        Ok(program_dir)
    }

    pub fn get_data_dir() -> Result<PathBuf, io::Error> {
        let mut program_dir = data_dir().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine data directory",
            )
        })?;
        program_dir.push("AGM");
        Ok(program_dir)
    }

    pub fn ensure_config_dirs() -> Result<(), io::Error> {
        let config_dir = Self::get_config_dir()?;
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let data_dir = Self::get_data_dir()?;
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }
        Ok(())
    }

    pub fn load() -> Result<Self, Error> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.yaml");

        if !config_file.exists() {
            return Ok(Self::new());
        }

        let raw_file_string = fs::read_to_string(&config_file)?;
        match serde_yaml::from_str(&raw_file_string) {
            Ok(config) => Ok(config),
            Err(e) => Err(Error::Yaml(e)),
        }
    }

    pub fn get_socket_path() -> Result<PathBuf, io::Error> {
        let mut data_dir = Self::get_data_dir()?;
        data_dir.push("agm.sock");
        Ok(data_dir)
    }

    pub fn save(&self) -> Result<(), Error> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.yaml");

        let yaml_string = serde_yaml::to_string(self)?;
        fs::write(config_file, yaml_string)?;
        Ok(())
    }
}
