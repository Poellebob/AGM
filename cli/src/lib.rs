use agm_core::ipc::{create_url_channel, start_ipc_server};
use agm_core::{install::InstallReporter, nexus, Agm};
pub use clap::{CommandFactory, Parser, Subcommand};
use std::io::{self, Write};
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub gui: bool,

    #[cfg(debug_assertions)]
    #[arg(long)]
    pub test: bool,

    #[cfg(debug_assertions)]
    #[arg(long)]
    pub test_url_handle: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Profile {
        #[command(subcommand)]
        cmd: CliProfile,
    },
    Preset {
        #[command(subcommand)]
        cmd: Preset,
    },
    /// Manage global application configuration
    Config(CliConfig),
    Install(Install),
}

#[derive(Parser, Debug)]
pub struct Install {
    /// Profile to use
    #[arg(long, required = true)]
    pub profile: String,

    /// Preset to use
    #[arg(long)]
    pub preset: Vec<String>,

    /// List of files to install
    #[arg(required = true)]
    pub files: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct CliConfig {
    /// Set the Nexus Mods API key
    #[arg(long)]
    pub nexus_api_key: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum CliProfile {
    List,

    Add { game: String },

    Edit { game: String },

    Remove { game: String },
}

#[derive(Subcommand, Debug)]
pub enum Preset {
    Switch {
        game: String,
        preset: String,
    },

    List {
        game: String,
    },

    Add {
        game: String,

        name: String,

        sources: Vec<String>,
    },

    Edit {
        game: String,
        preset: String,
    },

    Remove {
        game: String,

        preset: Option<String>,

        #[arg(short, long)]
        all: bool,
    },

    Disable {
        game: String,
    },
}

struct CliInstallReporter;

impl InstallReporter for CliInstallReporter {
    fn unpacking_start(&self, file_name: &str, dest: &str) {
        println!("Unpacking {} to {}", file_name, dest);
    }

    fn review_placements(&self, mod_name: &str) {
        println!("\nReviewing file placements for mod '{}':", mod_name);
    }

    fn prompt_for_point(&self, target: &str, moddirs: &[String]) -> io::Result<String> {
        println!("\n  File: {}", target);
        println!("  Automatic placement failed. Please choose a destination:");

        for (i, moddir) in moddirs.iter().enumerate() {
            println!("    {}) @{}", i + 1, moddir);
        }
        println!("    [c] Custom Path (e.g., game/data)");
        println!("    [s] Skip this file (leave point empty)");
        println!("    [q] Quit installation");

        loop {
            print!("  Your choice: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "q" {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Installation cancelled by user.",
                ));
            } else if input == "s" {
                return Ok("".to_string());
            } else if input == "c" {
                print!("  Enter custom path: ");
                io::stdout().flush()?;
                let mut custom_path = String::new();
                io::stdin().read_line(&mut custom_path)?;
                return Ok(custom_path.trim().to_string());
            } else if let Ok(choice_index) = input.parse::<usize>() {
                if choice_index > 0 && choice_index <= moddirs.len() {
                    return Ok(format!("@{}", moddirs[choice_index - 1]));
                } else {
                    println!("  Invalid number. Please try again.");
                }
            } else {
                println!("  Invalid input. Please choose a number, 'c', 's', or 'q'.");
            }
        }
    }
}

pub async fn run(args: Args) {
    #[cfg(debug_assertions)]
    if args.test_url_handle {
        println!("Running in test url handle mode.");
        if let Err(e) = run_url_handler().await {
            eprintln!("Error running url handler: {}", e);
        }
        return;
    }

    let mut agm = Agm::new();

    match args.command {
        Some(Command::Profile { cmd: cli_profile_cmd }) => match cli_profile_cmd {
            CliProfile::List => {
                agm.list_profiles();
            }

            CliProfile::Add { game } => {
                agm.add_profile(&game);
            }

            CliProfile::Edit { game } => {
                println!("Edit profile for game: {}", game);
            }

            CliProfile::Remove { game } => {
                agm.remove_profile(&game);
            }
        },

        Some(Command::Preset { cmd }) => match cmd {
            Preset::Switch { game, preset } => {
                println!("Switch preset for game '{}' to '{}'", game, preset);
            }

            Preset::List { game: _ } => {
                agm.list_presets();
            }

            Preset::Add {
                game: _,
                name,
                sources: _,
            } => {
                agm.add_preset(&name);
            }

            Preset::Edit { game, preset } => {
                println!("Edit preset '{}' for game '{}'", preset, game);
            }

            Preset::Remove { game, preset, all } => {
                if all {
                    println!("Delete ALL presets for game: {}", game);
                } else if let Some(preset) = preset {
                    agm.remove_preset(&preset);
                } else {
                    eprintln!("{}", "Error: no preset specified and --all not set");
                }
            }

            Preset::Disable { game } => {
                println!("Disable presets for game: {}", game);
            }
        },

        Some(Command::Config(cli_config_cmd)) => {
            if let Some(key) = cli_config_cmd.nexus_api_key {
                agm.set_nexus_api_key(&key);
                println!("Nexus API key set successfully.");
            } else {
                eprintln!("Error: No configuration option specified.");
            }
        }

        Some(Command::Install(cmd)) => {
            let reporter = CliInstallReporter;
            if let Err(e) = agm.install_mods(&cmd.files, &cmd.profile, &reporter).await {
                eprintln!("Error installing mods: {}", e);
            }
        }

        None => {
            Args::command().print_help().unwrap();
        }
    }
}

async fn run_url_handler() -> Result<(), Box<dyn std::error::Error + Send>> {
    let (url_sender, mut url_receiver) = create_url_channel();
    let port = 3000;

    let ipc_server_handle = tokio::spawn(start_ipc_server(url_sender, port));

    println!("IPC server started on port {}. Waiting for URLs...", port);

    if let Some(url_message) = url_receiver.recv().await {
        if let Ok(parsed_url) = Url::parse(&url_message.url) {
            if parsed_url.scheme() == "nxm" {
                let game = parsed_url.host_str().unwrap_or_default().to_string();
                let path_segments: Vec<&str> =
                    parsed_url.path_segments().map(|c| c.collect()).unwrap_or_default();

                if path_segments.len() == 4
                    && path_segments[0] == "mods"
                    && path_segments[2] == "files"
                {
                    let mod_id: u64 = path_segments[1].parse().unwrap_or(0);
                    let file_id: u64 = path_segments[3].parse().unwrap_or(0);

                    if mod_id > 0 && file_id > 0 {
                        let agm = Agm::new();
                        if let Some(api_key) = agm.get_nexus_api_key() {
                            match nexus::get_download_link(api_key, &game, mod_id, file_id).await {
                                Ok(link) => {
                                    println!("{}", link);
                                }
                                Err(e) => {
                                    eprintln!("Error getting download link: {}", e);
                                }
                            }
                        } else {
                            eprintln!("Nexus API key not set. Please set it using 'agm config --nexus-api-key <key>'");
                        }
                    }
                }
            }
        }
    }

    ipc_server_handle
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?
}
