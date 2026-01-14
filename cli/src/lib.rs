pub use clap::{Parser, Subcommand, CommandFactory};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub gui: bool,

    #[arg(long, default_value = "")]
    test: String,
}

pub fn exec(args: Args) {
    println!("{}", args.test)
}
