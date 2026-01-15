#[allow(unused_imports)]
use cli::{Args, Parser, CommandFactory};

fn main() {
    let args = Args::parse();
    let arg_count = std::env::args_os().len();

    if args.test {
        agmcore::core_logic();
    }

    if args.gui {
        #[cfg(feature = "gui")]
        gui::run();

        #[cfg(not(feature = "gui"))]
        eprintln!("gui feature is not enabled in this binary.");

        return;
    } else if arg_count == 1 {
        #[cfg(feature = "tui")]
        tui::run();

        #[cfg(all(not(feature = "tui"), feature = "gui"))]
        gui::run();

        #[cfg(all(not(feature = "tui"), not(feature = "gui")))]
        {
            cli::run(args);
        }
        return;
    } else {
        cli::run(args);
    }
}

