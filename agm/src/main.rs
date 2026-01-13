use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Launch the Terminal User Interface
    #[arg(long)]
    tui: bool,

    /// Launch the Graphical User Interface
    #[arg(long)]
    gui: bool,
}

fn main() {
    let args = Args::parse();

    if args.gui {
        #[cfg(feature = "gui")]
        {
            println!("Launching GUI...");
            gui::run();
        }
        #[cfg(not(feature = "gui"))]
        {
            println!("GUI feature is not enabled. Compile with --features gui");
        }
    } else if args.tui {
        #[cfg(feature = "tui")]
        {
            println!("Launching TUI...");
            tui::run();
        }
        #[cfg(not(feature = "tui"))]
        {
            println!("TUI feature is not enabled. Compile with --features tui");
        }
    } else {
        println!("Launching CLI...");
        cli::run();
    }
}

