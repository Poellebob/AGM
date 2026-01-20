use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Preset {
    pub name: String,
    pub mods: Vec<Mod>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mod {
    pub name: String,
}

impl Preset {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mods: Vec::new(),
        }
    }

    pub fn from_file(path: &Path) -> Self {
        let raw_file_string =
            fs::read_to_string(path).expect(&format!("Could not read file at {:?}", path));
        let preset: Preset = serde_yaml::from_str(&raw_file_string)
            .expect(&format!("Could not parse preset {:?}", path));

        preset
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}
