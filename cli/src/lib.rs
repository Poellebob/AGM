use agm_core::Agm;
use agm_core::ipc::{create_url_channel, start_ipc_server};
pub use clap::{CommandFactory, Parser, Subcommand};

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
        cmd: Profile,
    },
    Preset {
        #[command(subcommand)]
        cmd: Preset,
    },
}

#[derive(Subcommand, Debug)]
pub enum Profile {
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
        Some(Command::Profile { cmd }) => match cmd {
            Profile::List => {
                agm.list_profiles();
            }

            Profile::Add { game } => {
                agm.add_profile(&game);
            }

            Profile::Edit { game } => {
                println!("Edit profile for game: {}", game);
            }

            Profile::Remove { game } => {
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

    while let Some(url_message) = url_receiver.recv().await {
        println!("Received URL: {}", url_message.url);
    }

    ipc_server_handle
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?

}
