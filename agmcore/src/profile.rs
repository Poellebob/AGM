use serde::{Deserialize, Serialize};
use serde_yaml;
use std::path::Path;
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

pub fn get_profile(path: &Path) -> Profile {
    let raw_file_string = fs::read_to_string(path)
        .expect(&format!("Could not read file at {:?}", &path));
    let profile: Profile = serde_yaml::from_str(&raw_file_string)
        .expect(&format!("Could not parse profile {:?}", &path));

    profile
}
