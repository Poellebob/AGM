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

    pub fn get_nexus_api_key(&self) -> Option<&String> {
        self.config.nexus_api_key.as_ref()
    }

    pub fn list_profiles(&self) {
        println!("Profiles: {:?}", self.config.profiles);
    }

    pub fn add_profile(&mut self, name: &str) {
        self.config.profiles.push(name.to_string());
        self.config.save();
    }

    pub fn remove_profile(&mut self, name: &str) {
        self.config.profiles.retain(|p| p != name);
        self.config.save();
    }

    pub fn list_presets(&self) {
        println!("Presets: {:?}", self.config.presets);
    }

    pub fn add_preset(&mut self, name: &str) {
        self.config.presets.push(name.to_string());
        self.config.save();
    }

    pub fn remove_preset(&mut self, name: &str) {
        self.config.presets.retain(|p| p != name);
        self.config.save();
    }
}
