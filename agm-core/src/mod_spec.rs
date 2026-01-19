use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModSpec {
    pub name: String,
    pub url: Option<String>,
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub target: String,
    pub point: String,
}
