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
    pub mime: Option<Vec<String>>,
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

    pub fn get_moddir_names(&self) -> Vec<String> {
        let mut moddir_names = Vec::new();
        self.collect_moddir_names(&self.layout, &mut moddir_names);
        moddir_names
    }

    fn collect_moddir_names(&self, layouts: &[Layout], moddir_names: &mut Vec<String>) {
        for layout in layouts {
            if let LayoutType::Moddir = layout.node_type {
                moddir_names.push(layout.name.clone());
            }
            if let Some(sub_layouts) = &layout.sub {
                self.collect_moddir_names(sub_layouts, moddir_names);
            }
        }
    }
}

impl Layout {
    pub fn find_matching_moddir_point(&self, file_extension: &str) -> Option<String> {
        if let LayoutType::Moddir = self.node_type {
            if let Some(mimes) = &self.mime {
                if mimes.iter().any(|m| m == file_extension) {
                    return Some(format!("@{}", self.name));
                }
            }
        }

        if let Some(sub_layouts) = &self.sub {
            for sub_layout in sub_layouts {
                if let Some(point) = sub_layout.find_matching_moddir_point(file_extension) {
                    return Some(point);
                }
            }
        }
        None
    }
}
