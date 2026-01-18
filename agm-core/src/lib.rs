pub mod config;
pub mod ipc;
pub mod preset;
pub mod profile;
pub mod symlink;

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
