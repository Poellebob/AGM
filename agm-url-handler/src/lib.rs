use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlMessage {
    pub url: String,
    pub scheme: String,
    pub timestamp: u64,
}

pub fn get_socket_path() -> PathBuf {
    let mut program_dir = dirs_next::data_dir().unwrap();
    program_dir.push("AGM");
    program_dir.push("agm.sock");
    program_dir
}
