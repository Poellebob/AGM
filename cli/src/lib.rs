pub use clap::{Parser, Subcommand, CommandFactory};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub gui: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Profile {
        #[command(subcommand)]
        cmd: ProfileCmd,
    },
    Preset {
        #[command(subcommand)]
        cmd: PresetCmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProfileCmd {
    List,

    Add {
        game: String,
    },

    Edit {
        game: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum PresetCmd {
    Switch {
      game: String,
      preset: String,
    },

    List {
        game: String,
    },

    Add {
        game: String,

        sources: Vec<String>,
    },

    Edit {
        game: String,
        preset: String,
      },

    Delete {
        game: String,

        preset: Option<String>,

        #[arg(short, long)]
        all: bool,
    },

    Disable {
        game: String,
    },
}

pub fn run(args: Args) {
    match args.command {
        Some(Command::Profile { cmd }) => match cmd {
            ProfileCmd::List => {
                println!("{}", "List all game profiles");
            }

            ProfileCmd::Add { game } => {
                println!("Add profile for game: {}", game);
            }

            ProfileCmd::Edit { game } => {
                println!("Edit profile for game: {}", game);
            }
        },

        Some(Command::Preset { cmd }) => match cmd {
            PresetCmd::Switch { game, preset } => {
                println!("Switch preset for game '{}' to '{}'", game, preset);
            }

            PresetCmd::List { game } => {
                println!("List presets for game: {}", game);
            }

            PresetCmd::Add { game, sources } => {
                println!("Add preset for game: {}", game);
                println!("{}", "Sources:");
                for src in sources {
                    println!("    - {}", src);
                }
            }

            PresetCmd::Edit { game, preset } => {
                println!("Edit preset '{}' for game '{}'", preset, game);
            }

            PresetCmd::Delete { game, preset, all } => {
                if all {
                    println!("Delete ALL presets for game: {}", game);
                } else if let Some(preset) = preset {
                    println!("Delete preset '{}' for game '{}'", preset, game);
                } else {
                    eprintln!("{}", "Error: no preset specified and --all not set");
                }
            }

            PresetCmd::Disable { game } => {
                println!("Disable presets for game: {}", game);
            }
        },

        None => {
            Args::command().print_help().unwrap();
        }
    }
}
