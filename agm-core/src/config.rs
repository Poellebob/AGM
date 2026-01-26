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
pub struct GameConfig {
    pub profile: String,
    #[serde(default)]
    pub presets: Vec<String>,
    #[serde(default)]
    pub mods: Vec<String>,
    #[serde(default)]
    pub active_preset: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub games: Vec<GameConfig>,
    #[serde(default)]
    pub nexus_api_key: Option<String>,
    #[serde(default)]
    pub editor: Option<String>,

    // Legacy fields for migration
    #[serde(default, skip_serializing)]
    pub profiles: Vec<String>,
    #[serde(default, skip_serializing)]
    pub presets: Vec<LegacyPresetConfig>,
}

// Legacy structure for migration
#[derive(Debug, Deserialize, Serialize)]
pub struct LegacyPresetConfig {
    pub game: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub presets: Vec<String>,
    #[serde(default)]
    pub active_preset: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            games: Vec::new(),
            nexus_api_key: None,
            editor: None,
            profiles: Vec::new(),
            presets: Vec::new(),
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
        match serde_yaml::from_str::<Config>(&raw_file_string) {
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

    pub fn get_or_create_game(&mut self, profile_name: &str) -> &mut GameConfig {
        if let Some(index) = self.games.iter().position(|g| g.profile == profile_name) {
            &mut self.games[index]
        } else {
            self.games.push(GameConfig {
                profile: profile_name.to_string(),
                presets: Vec::new(),
                mods: Vec::new(),
                active_preset: None,
            });
            self.games.last_mut().unwrap()
        }
    }

    pub fn get_game(&self, profile_name: &str) -> Option<&GameConfig> {
        self.games.iter().find(|g| g.profile == profile_name)
    }

    pub fn get_game_mut(&mut self, profile_name: &str) -> Option<&mut GameConfig> {
        self.games.iter_mut().find(|g| g.profile == profile_name)
    }

    pub fn remove_game(&mut self, profile_name: &str) {
        self.games.retain(|g| g.profile != profile_name);
    }

    pub fn get_profile_names(&self) -> Vec<String> {
        self.games.iter().map(|g| g.profile.clone()).collect()
    }

    pub fn add_preset_to_game(&mut self, game_name: &str, preset_name: &str) {
        if let Some(game_config) = self.get_game_mut(game_name) {
            if !game_config.presets.contains(&preset_name.to_string()) {
                game_config.presets.push(preset_name.to_string());
            }
        } else {
            self.games.push(GameConfig {
                profile: game_name.to_string(),
                presets: vec![preset_name.to_string()],
                mods: Vec::new(),
                active_preset: None,
            });
        }
    }

    pub fn remove_preset_from_game(&mut self, game_name: &str, preset_name: &str) {
        if let Some(game_config) = self.get_game_mut(game_name) {
            game_config.presets.retain(|p| p != preset_name);
        }
    }

    pub fn add_mod_to_game(&mut self, game_name: &str, mod_name: &str) {
        if let Some(game_config) = self.get_game_mut(game_name) {
            if !game_config.mods.contains(&mod_name.to_string()) {
                game_config.mods.push(mod_name.to_string());
            }
        } else {
            self.games.push(GameConfig {
                profile: game_name.to_string(),
                presets: Vec::new(),
                mods: vec![mod_name.to_string()],
                active_preset: None,
            });
        }
    }

    pub fn remove_mod_from_game(&mut self, game_name: &str, mod_name: &str) {
        if let Some(game_config) = self.get_game_mut(game_name) {
            game_config.mods.retain(|m| m != mod_name);
        }
    }

    pub fn get_mods_for_game(&self, game_name: &str) -> Vec<String> {
        self.get_game(game_name)
            .map(|g| g.mods.clone())
            .unwrap_or_default()
    }
}
