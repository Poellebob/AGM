pub mod async_runtime;
pub mod config;
pub mod install;
pub mod ipc;
pub mod mod_spec;
pub mod nexus;
pub mod preset;
pub mod profile;
pub mod symlink;

use crate::install::{install_mods as core_install_mods, InstallReporter};
use crate::config::{Config, GameConfig};
use crate::ipc::{start_ipc_server, create_url_channel};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::preset::Preset;
use crate::profile::Profile;
use serde_yaml;
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Config error: {0}")]
    Config(#[from] crate::config::Error),
    #[error("Editor exited with a non-zero status")]
    Editor,
    #[error("Profile '{0}' not found")]
    ProfileNotFound(String),
    #[error("Preset '{0}' for game '{1}' not found")]
    PresetNotFound(String, String),
}

fn get_editor(config: &Config) -> String {
    if let Some(editor) = &config.editor {
        return editor.clone();
    }
    if let Ok(editor) = env::var("EDITOR") {
        return editor;
    }
    // Fallback editor
    if cfg!(target_os = "linux") {
        "nano".to_string()
    } else if cfg!(target_os = "windows") {
        "notepad".to_string()
    } else {
        "nano".to_string() // Default for other OS
    }
}

fn open_in_editor(editor: &str, file_path: &Path, content: Option<&str>) -> Result<(), Error> {
    if let Some(content) = content {
        std::fs::write(file_path, content)?;
        return Ok(());
    }
    
    let result = Command::new(editor).arg(file_path).status();
    
    match result {
        Ok(status) => {
            if status.success() {
                return Ok(());
            }

            eprintln!("Editor exited with non-zero status: {:?}", status);
            return Err(Error::Editor);
        }
        Err(e) => {
            eprintln!("Failed to run editor '{}': {}", editor, e);
            return Err(Error::Editor);
        }
    }
}

pub struct Agm {
    config: Config,
}

impl Agm {
    pub fn new() -> Result<Self, Error> {
        Config::ensure_config_dirs()?;
        let mut config = Config::load()?;
        
        // Sync mods from storage on startup
        Self::sync_mods_from_storage(&mut config)?;
        
        Ok(Self { config })
    }

    
    fn sync_mods_from_storage(config: &mut Config) -> Result<(), Error> {
        let storage_path = Config::get_data_dir()?.join("storage");
        if !storage_path.exists() {
            return Ok(());
        }

        for game_dir in fs::read_dir(storage_path)? {
            let game_dir = game_dir?;

            if !game_dir.file_type()?.is_dir() {
                continue;
            }

            let game_name = game_dir.file_name().to_str().map(|s| s.to_owned());
            let Some(game_name) = game_name else {
                continue;
            };

            let mut storage_mods = Vec::new();

            // Scan storage for mods
            for mod_entry in fs::read_dir(game_dir.path())? {
                let mod_entry = mod_entry?;

                if !mod_entry.file_type()?.is_dir() {
                    continue;
                }

                let mod_name = mod_entry.file_name().to_str().map(|s| s.to_owned());
                let Some(mod_name) = mod_name else {
                    continue;
                };

                storage_mods.push(mod_name.to_string());
            }

            // Update config with discovered mods
            let game_config = config.get_or_create_game(&game_name);
            game_config.mods = storage_mods;
        }

        config.save()?;
        Ok(())
    }


    pub async fn install_mods(
        &self,
        files: &[String],
        profile_name: &str,
        mod_name: &str,
        reporter: &dyn InstallReporter,
    ) -> std::io::Result<()> {
        core_install_mods(files, profile_name, mod_name, reporter).await
    }

    pub fn install_mods_blocking(
        &self,
        files: &[String],
        profile_name: &str,
        mod_name: &str,
        reporter: &dyn InstallReporter,
    ) -> std::io::Result<()> {
        async_runtime::run_blocking(core_install_mods(files, profile_name, mod_name, reporter))
    }

    pub fn activate_mod(&self, game: &str, mod_name: &str) -> Result<Vec<(PathBuf, PathBuf)>, Error> {
        let profile = self.get_profile_by_name(game)?.ok_or_else(|| Error::ProfileNotFound(game.to_string()))?;

        let storage_path = Config::get_data_dir()?.join("storage").join(game).join(mod_name);
        let mod_spec_path = storage_path.join(format!("{}.yaml", mod_name));

        if !mod_spec_path.exists() {
            return Ok(vec![]);
        }

        let mut symlinks = vec![];

        let mod_spec: crate::mod_spec::ModSpec = serde_yaml::from_str(&std::fs::read_to_string(&mod_spec_path)?)?;
        for file_entry in &mod_spec.files {
            if file_entry.point.is_empty() {
                return Ok(vec![]);
            }

            let Some(dest_dir_suffix) = profile.resolve_point(&file_entry.point) else {
                continue;
            };

            let source_path = storage_path.join(&file_entry.target);
            let dest_path = Path::new(&profile.game.path)
                .join(dest_dir_suffix)
                .join(&file_entry.target);

            crate::symlink::create_symlink(&source_path, &dest_path)?;
            symlinks.push((source_path, dest_path));
        }
        Ok(symlinks)
    }

    pub fn set_nexus_api_key(&mut self, key: &str) -> Result<(), Error> {
        self.config.nexus_api_key = Some(key.to_string());
        self.config.save()?;
        Ok(())
    }

    pub fn set_editor(&mut self, editor: &str) -> Result<(), Error> {
        self.config.editor = Some(editor.to_string());
        self.config.save()?;
        Ok(())
    }

    pub fn get_nexus_api_key(&self) -> Option<&String> {
        self.config.nexus_api_key.as_ref()
    }

    pub fn get_profile_names(&self) -> Vec<String> {
        self.config.get_profile_names()
    }

    fn get_profile_by_name(&self, name: &str) -> Result<Option<Profile>, Error> {
        if self.config.games.iter().any(|g| g.profile == name) {
            let profile_path = Config::get_data_dir()?.join("profiles").join(format!("{}.yaml", name));
            if profile_path.exists() {
                return Ok(Some(Profile::from_file(&profile_path)));
            }
        }
        Ok(None)
    }

    pub fn add_profile(&mut self, game: String, name: Option<String>, content: Option<String>, game_path: Option<String>) -> Result<(), Error> {
        let profile_name = name.unwrap_or_else(|| game.clone());
        let profile_path = Config::get_data_dir()?.join("profiles").join(format!("{}.yaml", profile_name));
    
        if profile_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Profile '{}' already exists.", profile_name),
            ).into());
        }
    
        let content_to_write = if let Some(c) = content {
            c
        } else if let Some(game_path) = game_path {
            let default_profile = Profile::new(&game, &game_path);
            serde_yaml::to_string(&default_profile)?
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Either content or game_path must be provided").into());
        };
        
        let profile_dir = Config::get_data_dir()?.join("profiles");
        if !profile_dir.exists() {
            std::fs::create_dir_all(&profile_dir)?;
        }
        
        std::fs::write(&profile_path, &content_to_write)?;
    
        // Add to new games structure
        self.config.get_or_create_game(&profile_name);
        self.config.save()?;
    
        // Automatically create a "vanilla" preset for the new profile
        self.add_preset(game.clone(), "vanilla".to_string(), None)?;
    
        Ok(())
    }

    pub fn edit_profile(&self, name: &str, content: Option<String>) -> Result<(), Error> {
        let profile_path = Config::get_data_dir()?.join("profiles").join(format!("{}.yaml", name));
        if !profile_path.exists() {
            return Err(Error::ProfileNotFound(name.to_string()));
        }

        let editor = get_editor(&self.config);
        open_in_editor(&editor, &profile_path, content.as_deref())
    }

    pub fn list_mods_for_game(&self, game: &str) -> Result<Vec<String>, Error> {
        let storage_path = Config::get_data_dir()?.join("storage").join(game);
        if !storage_path.exists() {
            return Ok(Vec::new());
        }

        let mut mods = Vec::new();
        for entry in fs::read_dir(storage_path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(mod_name) = entry.file_name().to_str() {
                    mods.push(mod_name.to_string());
                }
            }
        }
        Ok(mods)
    }

    /// Get mods for a game from config (fast lookup from tracked mods)
    pub fn get_mods(&self, game: &str) -> Vec<String> {
        self.config.get_mods_for_game(game)
    }

    pub fn remove_profile(&mut self, name: &str, remove_presets: bool, remove_mods: bool) -> Result<(), Error> {
        if remove_presets {
            if let Some(game_config) = self.config.get_game(name) {
                for preset in &game_config.presets.clone() {
                    self.remove_preset(name, preset)?;
                }
            }
        }

        if remove_mods {
            let mod_storage_path = Config::get_data_dir()?.join("storage").join(name);
            if mod_storage_path.exists() {
                fs::remove_dir_all(mod_storage_path)?;
            }
        }

        let profile_path = Config::get_data_dir()?.join("profiles").join(format!("{}.yaml", name));
        if profile_path.exists() {
            std::fs::remove_file(profile_path)?;
        }
        
        self.config.remove_game(name);
        self.config.save()?;
        
        Ok(())
    }

    pub fn get_presets(&self) -> &Vec<GameConfig> {
        &self.config.games
    }

    pub fn get_preset_names(&self, game: &str) -> Vec<String> {
        if let Some(game_config) = self.config.get_game(game) {
            game_config.presets.clone()
        } else {
            Vec::new()
        }
    }

    pub fn is_preset_active(&self, game: &str, preset: &str) -> bool {
        if let Some(game_config) = self.config.get_game(game) {
            game_config.active_preset.as_deref() == Some(preset)
        } else {
            false
        }
    }

    pub fn add_preset(&mut self, game: String, name: String, content: Option<String>) -> Result<(), Error> {
        let preset_path = Config::get_data_dir()?.join("presets").join(&game).join(format!("{}.yaml", name));
        
        let preset_dir = Config::get_data_dir()?.join("presets").join(&game);
        
        
        if preset_path.exists() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Preset '{}' for game '{}' already exists.", name, game)).into());
        }

        std::fs::create_dir_all(&preset_dir)?;

        let new_preset = crate::preset::Preset::new(&name);
        let yaml_string = serde_yaml::to_string(&new_preset)?;
        
        self.config.add_preset_to_game(&game, &name);
        self.config.save()?;

        let editor = get_editor(&self.config);
        open_in_editor(&editor, &preset_path, content.as_deref().or(Some(&yaml_string)))?;
        Ok(())
    }

    pub fn edit_preset(&mut self, game: &str, name: &str, content: Option<String>) -> Result<(), Error> {
        let preset_path = Config::get_data_dir()?.join("presets").join(game).join(format!("{}.yaml", name));
        if !preset_path.exists() {
             return Err(Error::PresetNotFound(name.to_string(), game.to_string()));
        }

        let editor = get_editor(&self.config);
        open_in_editor(&editor, &preset_path, content.as_deref())?;
        
        Ok(())
    }

    pub fn remove_preset(&mut self, game: &str, name: &str) -> Result<(), Error> {
        let is_active = self.is_preset_active(game, name);

        if is_active {
            self.deactivate_preset(game)?;
            if let Some(game_config) = self.config.get_game_mut(game) {
                game_config.active_preset = None;
            }
        }

        let preset_path = Config::get_data_dir()?.join("presets").join(game).join(format!("{}.yaml", name));
        if preset_path.exists() {
            std::fs::remove_file(preset_path)?;
        }

        self.config.remove_preset_from_game(game, name);

        self.config.save()?;
        Ok(())
    }

    pub fn add_mod_to_presets(&mut self, game: &str, mod_name: &str, presets: &[String]) -> Result<(), Error> {
        for preset_name in presets {
            let preset_path = Config::get_data_dir()?.join("presets").join(game).join(format!("{}.yaml", preset_name));
            
            if !preset_path.exists() {
                return Err(Error::PresetNotFound(preset_name.to_string(), game.to_string()));
            }

            let mut preset = Preset::from_file(&preset_path);
            preset.mods.push(crate::preset::Mod::Simple(mod_name.to_string()));
            let yaml_string = serde_yaml::to_string(&preset)?;
            std::fs::write(&preset_path, yaml_string)?;
        }
        Ok(())
    }

    pub fn add_mods_to_preset(&mut self, game: &str, preset_name: &str, mod_names: &[String]) -> Result<(), Error> {
        let preset_path = Config::get_data_dir()?.join("presets").join(game).join(format!("{}.yaml", preset_name));
        if !preset_path.exists() {
            return Err(Error::PresetNotFound(preset_name.to_string(), game.to_string()));
        }

        let mut preset = Preset::from_file(&preset_path);
        for mod_name in mod_names {
            // Avoid duplicates
            if !preset.mods.iter().any(|m| match m {
                preset::Mod::Simple(name) => name == mod_name,
                preset::Mod::Detailed(info) => &info.name == mod_name,
            }) {
                preset.mods.push(crate::preset::Mod::Simple(mod_name.to_string()));
            }
        }

        let yaml_string = serde_yaml::to_string(&preset)?;
        std::fs::write(&preset_path, yaml_string)?;
        Ok(())
    }

    pub fn remove_mod_from_preset(
        &mut self, 
        game: &str, 
        preset: &str,
        name: &str
    ) -> Result<(), Error> {
        let _game_config = self.config.get_game(game)
            .ok_or_else(|| Error::ProfileNotFound(format!("Game '{}'", game)))?;

        let preset_path = Config::get_data_dir()?.join("presets").join(game).join(format!("{}.yaml", preset));
        if !preset_path.exists() {
            return Ok(());
        }

        let mut preset = Preset::from_file(&preset_path);
        preset.mods.retain(|m| match m {
            crate::preset::Mod::Simple(mod_name) => mod_name != name,
            crate::preset::Mod::Detailed(info) => info.name != name,
        });
        let yaml_string = serde_yaml::to_string(&preset)?;
        std::fs::write(&preset_path, yaml_string)?;
        Ok(())
    }

    pub fn remove_mod(&mut self, game: &str, name: &str, purge: bool) -> Result<(), Error> {
        // Find the game configuration for the specified game
        let game_config = self.config.get_game(game)
            .ok_or_else(|| Error::ProfileNotFound(format!("Game '{}'", game)))?;

        let presets = game_config.presets.clone();
        for preset in &presets {
            self.remove_mod_from_preset(game, preset, name)?;
        }

        // Remove from config
        self.config.remove_mod_from_game(game, name);
        self.config.save()?;

        // If purge is requested, remove mod from storage for this specific game only
        if purge {
            let mod_storage_path = Config::get_data_dir()?.join("storage").join(game).join(name);
            if mod_storage_path.exists() {
                fs::remove_dir_all(mod_storage_path)?;
            }
        }
        
        Ok(())
    }

    
    pub fn switch_preset(&mut self, game: &str, preset: &str) -> Result<(), Error> {
        self.deactivate_preset(game)?;
        self.activate_preset(game, preset)?;

        if self.config.get_game(game).is_none() {
            self.config.games.push(config::GameConfig {
                profile: game.to_string(),
                presets: vec![preset.to_string()],
                mods: vec![],
                active_preset: Some(preset.to_string()),
            });
        }

        let game_config = self
            .config
            .get_game_mut(game)
            .expect("game was just inserted");

        game_config.active_preset = Some(preset.to_string());

        self.config.save()?;
        Ok(())
    }

    

    fn deactivate_preset(&mut self, game: &str) -> Result<Vec<PathBuf>, Error> {
        let mut removed_symlinks = vec![];

        let game_config = match self.config.get_game(game) {
            Some(gc) => gc,
            None => return Ok(removed_symlinks),
        };

        let active_preset_name = match &game_config.active_preset {
            Some(name) => name,
            None => return Ok(removed_symlinks),
        };

        let profile = self
            .get_profile_by_name(game)?
            .ok_or_else(|| Error::ProfileNotFound(game.to_string()))?;

        let preset_path = Config::get_data_dir()?
            .join("presets")
            .join(game)
            .join(format!("{}.yaml", active_preset_name));

        let preset = Preset::from_file(&preset_path);

        for mod_entry in &preset.mods {
            let mod_name = match mod_entry {
                preset::Mod::Simple(name) => name,
                preset::Mod::Detailed(info) => &info.name,
            };

            let mod_spec_path = Config::get_data_dir()?
                .join("storage")
                .join(game)
                .join(mod_name)
                .join(format!("{}.yaml", mod_name));

            if !mod_spec_path.exists() {
                continue;
            }

            let mod_spec: crate::mod_spec::ModSpec =
            serde_yaml::from_str(&std::fs::read_to_string(&mod_spec_path)?)?;

            for file_entry in &mod_spec.files {
            if file_entry.point.is_empty() {
                continue;
            }

            let dest_dir_suffix = match profile.resolve_point(&file_entry.point) {
                Some(p) => p,
                None => continue,
            };

            let dest_path = Path::new(&profile.game.path)
                .join(dest_dir_suffix)
                .join(&file_entry.target);

            if !dest_path.exists() || !dest_path.is_symlink() {
                continue;
            }

            std::fs::remove_file(&dest_path)?;
                removed_symlinks.push(dest_path);
            }
        }

        Ok(removed_symlinks)
    }


    fn activate_preset(&mut self, game: &str, preset_name: &str) -> Result<Vec<(PathBuf, PathBuf)>, Error> {
        let _profile = self.get_profile_by_name(game)?.ok_or_else(|| Error::ProfileNotFound(game.to_string()))?;

        let preset_path = Config::get_data_dir()?.join("presets").join(game).join(format!("{}.yaml", preset_name));
        if !preset_path.exists() {
            return Err(Error::PresetNotFound(preset_name.to_string(), game.to_string()));
        }
        let preset = Preset::from_file(&preset_path);
        let mut created_symlinks = vec![];

        for mod_entry in &preset.mods {
            let mod_name = match mod_entry {
                preset::Mod::Simple(name) => name,
                preset::Mod::Detailed(info) => &info.name,
            };
            created_symlinks.extend(self.activate_mod(game, mod_name)?);
        }
        Ok(created_symlinks)
    }
}

pub async fn run_url_handler() -> Result<(), Box<dyn std::error::Error + Send>> {
    let (url_sender, mut url_receiver) = create_url_channel();
    let port = 3000;

    let ipc_server_handle = tokio::spawn(start_ipc_server(url_sender, port));

    println!("IPC server started on port {}. Waiting for URLs...", port);

    if let Some(url_message) = url_receiver.recv().await {
        if let Ok(parsed_url) = Url::parse(&url_message.url) {
            if parsed_url.scheme() == "nxm" {
                let game = parsed_url.host_str().unwrap_or_default().to_string();
                let path_segments: Vec<&str> =
                    parsed_url.path_segments().map(|c| c.collect()).unwrap_or_default();

                if path_segments.len() == 4
                    && path_segments[0] == "mods"
                    && path_segments[2] == "files"
                {
                    let mod_id: u64 = path_segments[1].parse().unwrap_or(0);
                    let file_id: u64 = path_segments[3].parse().unwrap_or(0);

                    if mod_id > 0 && file_id > 0 {
                        let agm = match Agm::new() {
                            Ok(agm) => agm,
                            Err(e) => {
                                eprintln!("Error initializing AGM: {}", e);
                                return Err(Box::new(e));
                            }
                        };
                        if let Some(api_key) = agm.get_nexus_api_key() {
                            match nexus::get_download_link(api_key, &game, mod_id, file_id).await {
                                Ok(link) => {
                                    println!("{}", link);
                                }
                                Err(e) => {
                                    eprintln!("Error getting download link: {}", e);
                                }
                            }
                        } else {
                            eprintln!("Nexus API key not set. Please set it using 'agm config --nexus-api-key <key>'");
                        }
                    }
                }
            }
        }
    }

    ipc_server_handle
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?
}
