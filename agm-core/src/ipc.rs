use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlMessage {
    pub url: String,
    pub scheme: String,
    pub timestamp: u64,
}
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;
use tokio::sync::mpsc;

pub type UrlSender = mpsc::UnboundedSender<UrlMessage>;
pub type UrlReceiver = mpsc::UnboundedReceiver<UrlMessage>;

/// Creates a channel for URL messages
pub fn create_url_channel() -> (UrlSender, UrlReceiver) {
    mpsc::unbounded_channel()
}

/// Starts the IPC server that listens for URLs from agm-url-handler
pub async fn start_ipc_server(
    url_sender: UrlSender,
    _port: u16, // port is no longer used, but kept for compatibility with calling code
) -> Result<(), Box<dyn std::error::Error + Send>> {
    let socket_path = Config::get_socket_path().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
    // Ensure the socket file does not exist
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
    }

    let listener = UnixListener::bind(&socket_path)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

    println!("IPC server listening on {:?}", socket_path);

    loop {
        let (mut stream, _) = listener
            .accept()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
        let url_sender_clone = url_sender.clone();

        tokio::spawn(async move {
            let mut buffer = Vec::new();
            if let Err(e) = stream.read_to_end(&mut buffer).await {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }

            match serde_json::from_slice::<UrlMessage>(&buffer) {
                Ok(msg) => {
                    if let Err(e) = url_sender_clone.send(msg) {
                        eprintln!("Failed to send URL message: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to deserialize message: {}", e);
                }
            }
        });
    }
}