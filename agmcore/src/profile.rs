use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub game: Game,
    pub layout: Vec<Layout>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: Type,

    // Optional because not every node has `sub`
    pub sub: Option<Vec<LayoutNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Dir,
    Moddir,
}

pub fn get_profile(path: String) -> Profile {
    let rawFileString = fs::read_to_string(path)
        .expect("Could not read file at {}", path);
    let profile: Profile = serde_yaml::from_str(rawFileString);

    Ok(profile)
}
