use tokio::{fs::File, io::AsyncWriteExt};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DownloadLink {
    uri: String,
}

pub async fn get_download_link(
    api_key: &str,
    game: &str,
    mod_id: u64,
    file_id: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.nexusmods.com/v1/games/{}/mods/{}/files/{}/download_link.json",
        game, mod_id, file_id
    );

    let client = reqwest::Client::new();

    let resp = client
        .get(&url)
        .header("apikey", api_key)
        .send()
        .await?;

    if resp.status() == reqwest::StatusCode::FORBIDDEN {
        return Err("Direct downloads require Nexus Premium".into());
    }

    let links: Vec<DownloadLink> = resp.json().await?;
    Ok(links[0].uri.clone())
}
