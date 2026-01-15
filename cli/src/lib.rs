pub use clap::{Parser, Subcommand, CommandFactory};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub gui: bool,

    #[arg(long)]
    pub test: bool,

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

    Add {
        game: String,
    },

    Edit {
        game: String,
    },
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
            Profile::List => {
                println!("{}", "List all game profiles");
            }

            Profile::Add { game } => {
                println!("Add profile for game: {}", game);
            }

            Profile::Edit { game } => {
                println!("Edit profile for game: {}", game);
            }
        },

        Some(Command::Preset { cmd }) => match cmd {
            Preset::Switch { game, preset } => {
                println!("Switch preset for game '{}' to '{}'", game, preset);
            }

            Preset::List { game } => {
                println!("List presets for game: {}", game);
            }

            Preset::Add { game, name, sources } => {
                println!("Add preset named {} for game: {}",name, game);
                println!("{}", "Sources:");
                for src in sources {
                    println!("    - {}", src);
                }
            }

            Preset::Edit { game, preset } => {
                println!("Edit preset '{}' for game '{}'", preset, game);
            }

            Preset::Delete { game, preset, all } => {
                if all {
                    println!("Delete ALL presets for game: {}", game);
                } else if let Some(preset) = preset {
                    println!("Delete preset '{}' for game '{}'", preset, game);
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
