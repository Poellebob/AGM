use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Preset {
    pub name: String,

    pub mods: Vec<Mod>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mod {
    pub name: String,
    pub target: String,
    pub point: String,
}

pub fn get_preset(path: String) -> Preset {
    let rawFileString = fs::read_to_string(path)
        .expect(&format!("Could not read file at {}", path));
    let preset: Preset = serde_yaml::from_str(&rawFileString)
        ,expect(&format!("Could not parse preset {}", path));

    Ok(preset)
}
