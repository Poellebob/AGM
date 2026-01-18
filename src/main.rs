#[allow(unused_imports)]
use cli::{Args, CommandFactory, Parser};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.gui {
        #[cfg(feature = "gui")]
        gui::run();

        #[cfg(not(feature = "gui"))]
        eprintln!("gui feature is not enabled in this binary.");

        return;
    } else if std::env::args_os().len() == 1 {
        #[cfg(feature = "tui")]
        tui::run();

        #[cfg(all(not(feature = "tui"), feature = "gui"))]
        gui::run();

        #[cfg(all(not(feature = "tui"), not(feature = "gui")))]
        {
            cli::run(args).await;
        }
        return;
    } else {
        cli::run(args).await;
    }
}
