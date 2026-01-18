use agm_url_handler::{get_socket_path, UrlMessage};
use clap::Parser;
use std::process;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;
use url::Url;

mod register;

#[derive(Parser, Debug)]
#[command(name = "agm-url-handler")]
#[command(about = "AGM URL Handler - Captures and forwards URLs to AGM", long_about = None)]
struct Args {
    /// URL to handle (e.g., nxm://...)
    url: Option<String>,

    /// Register this program as nxm:// handler
    #[arg(long)]
    install: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.install {
        if let Err(e) = register::register_nxm_handler() {
            eprintln!("Install failed: {}", e);
            process::exit(1);
        }

        println!("NXM handler registered successfully.");
        return;
    }

    let url = match &args.url {
        Some(u) => u,
        None => {
            eprintln!("No URL provided.");
            process::exit(1);
        }
    };

    if let Err(e) = handle_url(url).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn handle_url(url_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse and validate URL
    let parsed_url = Url::parse(url_str)?;
    let scheme = parsed_url.scheme().to_string();

    // Validate supported schemes
    if !is_supported_scheme(&scheme) {
        return Err(format!("Unsupported URL scheme: {}", scheme).into());
    }

    println!("  Captured {} URL: {}", scheme.to_uppercase(), url_str);

    // Create message payload
    let message = UrlMessage {
        url: url_str.to_string(),
        scheme: scheme.clone(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
    };

    // Send to AGM
    send_to_agm(&message).await?;

    println!("  Successfully sent to AGM");
    Ok(())
}

fn is_supported_scheme(scheme: &str) -> bool {
    matches!(scheme, "nxm" | "nexusmods")
}

async fn send_to_agm(message: &UrlMessage) -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = get_socket_path();
    let mut stream = UnixStream::connect(&socket_path).await?;
    let message_json = serde_json::to_vec(message)?;

    stream.write_all(&message_json).await?;

    Ok(())
}
