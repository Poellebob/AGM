use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::Path;

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
    pub node_type: LayoutType,
    pub sub: Option<Vec<Layout>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayoutType {
    Dir,
    Moddir,
}

impl Profile {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            game: Game {
                name: name.to_string(),
                path: path.to_string(),
            },
            layout: Vec::new(),
        }
    }

    pub fn from_file(path: &Path) -> Self {
        let raw_file_string =
            fs::read_to_string(path).expect(&format!("Could not read file at {:?}", path));
        let profile: Profile = serde_yaml::from_str(&raw_file_string)
            .expect(&format!("Could not parse profile {:?}", path));

        profile
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}
