use agmcore::core_logic;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds a new mod
    Add { name: String },
    /// Removes a mod
    Remove { name: String },
}

pub fn run() {
    let cli = Cli::parse();
    core_logic();

    match &cli.command {
        Commands::Add { name } => {
            println!("'{}' was added", name);
        }
        Commands::Remove { name } => {
            println!("'{}' was removed", name);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // This is a placeholder test.
        // A real test would involve calling run() with specific arguments,
        // which requires more complex setup.
        assert_eq!(2 + 2, 4);
    }
}
