pub mod config;
pub mod install;
pub mod ipc;
pub mod mod_spec;
pub mod nexus;
pub mod preset;
pub mod profile;
pub mod symlink;

use crate::install::{install_mods as core_install_mods, InstallReporter};
use config::Config;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use crate::preset::Preset;
use crate::profile::Profile;
use serde_yaml;


fn get_editor(config: &Config) -> String {
    if let Some(editor) = &config.editor {
        return editor.clone();
    }
    if let Ok(editor) = env::var("EDITOR") {
        return editor;
    }
    // Fallback editor
    "vi".to_string()
}

fn open_in_editor(editor: &str, file_path: &Path) -> io::Result<()> {
    let status = Command::new(editor).arg(file_path).status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Editor '{}' exited with a non-zero status.", editor),
        ));
    }
    Ok(())
}

pub struct Agm {
    config: Config,
}

impl Agm {
    pub fn new() -> Self {
        Config::ensure_config_dirs();
        Self {
            config: Config::load(),
        }
    }

    pub async fn install_mods(
        &self,
        files: &[String],
        profile_name: &str,
        reporter: &dyn InstallReporter,
    ) -> std::io::Result<()> {
        core_install_mods(files, profile_name, reporter).await
    }

    pub fn set_nexus_api_key(&mut self, key: &str) {
        self.config.nexus_api_key = Some(key.to_string());
        self.config.save();
    }

    pub fn set_editor(&mut self, editor: &str) {
        self.config.editor = Some(editor.to_string());
        self.config.save();
    }

    pub fn get_nexus_api_key(&self) -> Option<&String> {
        self.config.nexus_api_key.as_ref()
    }

    fn get_profile_by_name(&self, name: &str) -> io::Result<Option<Profile>> {
        if self.config.profiles.iter().any(|p| p == name) {
            let profile_path = Config::get_data_dir().join("profiles").join(format!("{}.yaml", name));
            if profile_path.exists() {
                return Ok(Some(Profile::from_file(&profile_path)));
            }
        }
        Ok(None)
    }

    pub fn list_profiles(&self) {
        println!("Profiles: {:?}", self.config.profiles);
    }

    pub fn add_profile(&mut self, game: String, name: Option<String>) -> io::Result<()> {
        let profile_name = name.unwrap_or_else(|| game.clone());
        let profile_path = Config::get_data_dir().join("profiles").join(format!("{}.yaml", profile_name));

        if profile_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Profile '{}' already exists.", profile_name),
            ));
        }

        println!("Please enter the full path to the game's executable:");
        let mut game_path = String::new();
        io::stdin().read_line(&mut game_path)?;
        let game_path = game_path.trim().to_string();

        let default_profile = Profile::new(&game, &game_path);
        let yaml_string = serde_yaml::to_string(&default_profile).unwrap();
        
        let profile_dir = Config::get_data_dir().join("profiles");
        if !profile_dir.exists() {
            std::fs::create_dir_all(&profile_dir)?;
        }
        
        std::fs::write(&profile_path, yaml_string)?;

        self.config.profiles.push(profile_name.clone());
        self.config.save();

        println!("Created profile '{}'.", profile_name);
        println!("Opening profile in editor...");

        let editor = get_editor(&self.config);
        open_in_editor(&editor, &profile_path)
    }

    pub fn edit_profile(&self, name: &str) -> io::Result<()> {
        let profile_path = Config::get_data_dir().join("profiles").join(format!("{}.yaml", name));
        if !profile_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Profile '{}' not found.", name),
            ));
        }

        let editor = get_editor(&self.config);
        open_in_editor(&editor, &profile_path)
    }

    pub fn remove_profile(&mut self, name: &str) -> io::Result<()> {
        let profile_path = Config::get_data_dir().join("profiles").join(format!("{}.yaml", name));
        if profile_path.exists() {
            std::fs::remove_file(profile_path)?;
        }
        
        self.config.profiles.retain(|p| p != name);
        self.config.save();
        
        println!("Removed profile '{}'.", name);
        Ok(())
    }

    pub fn list_presets(&self, for_game: Option<String>) {
        if let Some(game_name) = for_game {
            if let Some(preset_config) = self.config.presets.iter().find(|p| p.game == game_name) {
                println!("Presets for {}:", game_name);
                for preset in &preset_config.presets {
                    if Some(preset) == preset_config.active_preset.as_ref() {
                        println!("  - {} (active)", preset);
                    } else {
                        println!("  - {}", preset);
                    }
                }
            } else {
                println!("No presets found for game '{}'.", game_name);
            }
        } else {
            println!("All configured presets:");
            for preset_config in &self.config.presets {
                println!("- {}:", preset_config.game);
                for preset in &preset_config.presets {
                     if Some(preset) == preset_config.active_preset.as_ref() {
                        println!("    - {} (active)", preset);
                    } else {
                        println!("    - {}", preset);
                    }
                }
            }
        }
    }

    pub fn add_preset(&mut self, game: String, name: String) -> io::Result<()> {
        let preset_path = Config::get_data_dir().join("presets").join(&game).join(format!("{}.yaml", name));
        
        let preset_dir = Config::get_data_dir().join("presets").join(&game);
        if !preset_dir.exists() {
            std::fs::create_dir_all(&preset_dir)?;
        }

        if preset_path.exists() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Preset '{}' for game '{}' already exists.", name, game)));
        }

        let new_preset = crate::preset::Preset::new(&name);
        let yaml_string = serde_yaml::to_string(&new_preset).unwrap();
        std::fs::write(&preset_path, yaml_string)?;
        
        if let Some(preset_config) = self.config.presets.iter_mut().find(|p| p.game == game) {
            preset_config.presets.push(name.clone());
        } else {
            self.config.presets.push(config::PresetConfig {
                game,
                aliases: vec![],
                presets: vec![name.clone()],
                active_preset: None,
            });
        }
        self.config.save();

        println!("Created preset '{}' for game '{}'.", name, "game");
        println!("Opening preset in editor...");
        let editor = get_editor(&self.config);
        open_in_editor(&editor, &preset_path)
    }

    pub fn edit_preset(&self, game: &str, name: &str) -> io::Result<()> {
        let preset_path = Config::get_data_dir().join("presets").join(game).join(format!("{}.yaml", name));
        if !preset_path.exists() {
             return Err(io::Error::new(io::ErrorKind::NotFound, format!("Preset '{}' for game '{}' not found.", name, game)));
        }

        let editor = get_editor(&self.config);
        open_in_editor(&editor, &preset_path)
    }

    pub fn remove_preset(&mut self, game: &str, name: &str) -> io::Result<()> {
        let preset_path = Config::get_data_dir().join("presets").join(game).join(format!("{}.yaml", name));
        if preset_path.exists() {
            std::fs::remove_file(preset_path)?;
        }

        if let Some(preset_config) = self.config.presets.iter_mut().find(|p| p.game == game) {
            preset_config.presets.retain(|p| p != name);
        }

        self.config.save();
        println!("Removed preset '{}' for game '{}'.", name, game);
        Ok(())
    }

    pub fn switch_preset(&mut self, game: &str, preset: &str) -> io::Result<()> {
        self.deactivate_preset(game)?;
        self.activate_preset(game, preset)?;
        
        if let Some(preset_config) = self.config.presets.iter_mut().find(|p| p.game == game) {
            preset_config.active_preset = Some(preset.to_string());
        } else {
            // This case should ideally not be hit if `activate_preset` was successful, but as a fallback:
            self.config.presets.push(config::PresetConfig {
                game: game.to_string(),
                aliases: vec![],
                presets: vec![preset.to_string()],
                active_preset: Some(preset.to_string()),
            });
        }

        self.config.save();
        println!("Switched to preset '{}' for game '{}'.", preset, game);
        Ok(())
    }
    
    fn deactivate_preset(&mut self, game: &str) -> io::Result<()> {
        if let Some(preset_config) = self.config.presets.iter().find(|p| p.game == game) {
            if let Some(active_preset_name) = &preset_config.active_preset {
                println!("Deactivating preset '{}' for game '{}'...", active_preset_name, game);
                let profile = match self.get_profile_by_name(game)? {
                    Some(p) => p,
                    None => return Err(io::Error::new(io::ErrorKind::NotFound, format!("Profile for game '{}' not found.", game))),
                };

                let preset_path = Config::get_data_dir().join("presets").join(game).join(format!("{}.yaml", active_preset_name));
                let preset = Preset::from_file(&preset_path);

                for mod_entry in &preset.mods {
                    let mod_name = &mod_entry.name;
                    let mod_spec_path = Config::get_data_dir().join("storage").join(game).join(mod_name).join(format!("{}.yaml", mod_name));
                    if mod_spec_path.exists() {
                        let mod_spec: crate::mod_spec::ModSpec = serde_yaml::from_str(&std::fs::read_to_string(&mod_spec_path)?).unwrap();
                        for file_entry in &mod_spec.files {
                            if !file_entry.point.is_empty() {
                                if let Some(dest_dir_suffix) = profile.resolve_point(&file_entry.point) {
                                    let dest_path = Path::new(&profile.game.path).join(dest_dir_suffix).join(&file_entry.target);
                                    if dest_path.exists() {
                                        if dest_path.is_symlink() {
                                            std::fs::remove_file(&dest_path)?;
                                            println!("Removed symlink: {}", dest_path.display());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn activate_preset(&mut self, game: &str, preset_name: &str) -> io::Result<()> {
        println!("Activating preset '{}' for game '{}'...", preset_name, game);
        let profile = match self.get_profile_by_name(game)? {
            Some(p) => p,
            None => return Err(io::Error::new(io::ErrorKind::NotFound, format!("Profile for game '{}' not found.", game))),
        };

        let preset_path = Config::get_data_dir().join("presets").join(game).join(format!("{}.yaml", preset_name));
        if !preset_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Preset '{}' for game '{}' not found.", preset_name, game)));
        }
        let preset = Preset::from_file(&preset_path);

        for mod_entry in &preset.mods {
            let mod_name = &mod_entry.name;
            let storage_path = Config::get_data_dir().join("storage").join(game).join(mod_name);
            let mod_spec_path = storage_path.join(format!("{}.yaml", mod_name));
            if mod_spec_path.exists() {
                let mod_spec: crate::mod_spec::ModSpec = serde_yaml::from_str(&std::fs::read_to_string(&mod_spec_path)?).unwrap();
                for file_entry in &mod_spec.files {
                    if !file_entry.point.is_empty() {
                        if let Some(dest_dir_suffix) = profile.resolve_point(&file_entry.point) {
                            let source_path = storage_path.join(&file_entry.target);
                            let dest_path = Path::new(&profile.game.path).join(dest_dir_suffix).join(&file_entry.target);
                             if let Err(e) = crate::symlink::create_symlink(&source_path, &dest_path) {
                                eprintln!("Failed to create symlink from {} to {}: {}", source_path.display(), dest_path.display(), e);
                            } else {
                                println!("Created symlink: {} -> {}", dest_path.display(), source_path.display());
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
