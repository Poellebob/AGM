use agm_core::ipc::{create_url_channel, start_ipc_server};
use agm_core::{install::InstallReporter, nexus, Agm};
pub use clap::{CommandFactory, Parser, Subcommand};
use std::io::{self, Write};
use std::path::Path;
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
    Mod {
        #[command(subcommand)]
        cmd: CliMod,
    },
}

#[derive(Subcommand, Debug)]
pub enum CliMod {
    Install(Install),
    Remove {
        name: String,
        #[arg(long)]
        purge: bool,
    },
}

#[derive(Parser, Debug)]
pub struct Install {
    /// Profile to use
    #[arg(long)]
    pub profile: Option<String>,

    /// Optional name for the mod
    #[arg(long)]
    pub name: Option<String>,

    /// List of files to install
    #[arg(required = true)]
    pub files: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct CliConfig {
    /// Set the Nexus Mods API key
    #[arg(long)]
    pub nexus_api_key: Option<String>,

    /// Set the editor to use for editing profiles
    #[arg(long)]
    pub editor: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum CliProfile {
    List,

    Add {
        game: String,
        #[arg(long)]
        name: Option<String>,
        content: Option<String>,
    },

    Edit {
        game: String,
        content: Option<String>,
    },

    Remove { game: String },
}

#[derive(Subcommand, Debug)]
pub enum Preset {
    Switch {
        game: String,
        preset: String,
    },

    List {
        #[arg(long)]
        profile: Option<String>,
    },

    Add {
        game: String,
        name: String,
        content: Option<String>,
    },

    Edit {
        game: String,
        name: String,
        content: Option<String>,
    },

    Remove {
        game: String,
        name: String,
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

    fn symlink_created(&self, source: &Path, destination: &Path) {
        println!(
            "  Created symlink: {} -> {}",
            destination.display(),
            source.display()
        );
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

    fn prompt_for_unpack(&self, file_name: &str) -> io::Result<bool> {
        print!("Do you want to unpack '{}'? (y/N): ", file_name);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        Ok(input == "y" || input == "Y")
    }

    fn prompt_for_profile(&self, profiles: &[String]) -> io::Result<String> {
        println!("Please choose a profile:");
        for (i, profile) in profiles.iter().enumerate() {
            println!("    {}) {}", i + 1, profile);
        }
        loop {
            print!("Your choice: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if let Ok(choice_index) = input.parse::<usize>() {
                if choice_index > 0 && choice_index <= profiles.len() {
                    return Ok(profiles[choice_index - 1].clone());
                } else {
                    println!("Invalid number. Please try again.");
                }
            } else {
                println!("Invalid input. Please choose a number.");
            }
        }
    }

    fn prompt_for_mod_name(&self, default_name: &str) -> io::Result<String> {
        print!("Enter a name for the mod (leave blank to use '{}'): ", default_name);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.is_empty() {
            Ok(default_name.to_string())
        } else {
            Ok(input.to_string())
        }
    }

    fn confirm_preset_add(&self) -> io::Result<bool> {
        print!("Do you want to add this mod to a preset? (y/N): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        Ok(input == "y" || input == "Y")
    }

    fn prompt_for_presets(&self, presets: &[String]) -> io::Result<Vec<String>> {
        println!("Please choose presets to add the mod to (e.g., 1 3):");
        for (i, preset) in presets.iter().enumerate() {
            println!("    {}) {}", i + 1, preset);
        }
        print!("Your choice: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let selected_presets = input
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .filter_map(|i| presets.get(i - 1).cloned())
            .collect();
            
        Ok(selected_presets)
    }

    fn confirm_profile_parts_removal(&self) -> io::Result<(bool, bool)> {
        print!("Do you want to remove all associated presets? (y/N): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let remove_presets = input.trim().eq_ignore_ascii_case("y");

        print!("Do you want to remove all associated mod files from storage? (y/N): ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let remove_mods = input.trim().eq_ignore_ascii_case("y");

        Ok((remove_presets, remove_mods))
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
            CliProfile::Add { game, name, content } => {
                if let Err(e) = agm.add_profile(game, name, content) {
                    eprintln!("Error adding profile: {}", e);
                }
            }
            CliProfile::Edit { game, content } => {
                if let Err(e) = agm.edit_profile(&game, content) {
                    eprintln!("Error editing profile: {}", e);
                }
            }
            CliProfile::Remove { game } => {
                let reporter = CliInstallReporter;
                if let Ok((remove_presets, remove_mods)) = reporter.confirm_profile_parts_removal() {
                    if let Err(e) = agm.remove_profile(&game, remove_presets, remove_mods) {
                        eprintln!("Error removing profile: {}", e);
                    }
                }
            }
        },

        Some(Command::Preset { cmd }) => match cmd {
            Preset::Switch { game, preset } => {
                if let Err(e) = agm.switch_preset(&game, &preset) {
                    eprintln!("Error switching preset: {}", e);
                }
            }
            Preset::List { profile } => {
                agm.list_presets(profile);
            }
            Preset::Add { game, name, content } => {
                if let Err(e) = agm.add_preset(game, name, content) {
                    eprintln!("Error adding preset: {}", e);
                }
            }
            Preset::Edit { game, name, content } => {
                if let Err(e) = agm.edit_preset(&game, &name, content) {
                    eprintln!("Error editing preset: {}", e);
                }
            }
            Preset::Remove { game, name } => {
                if let Err(e) = agm.remove_preset(&game, &name) {
                    eprintln!("Error removing preset: {}", e);
                }
            }
        },

        Some(Command::Config(cli_config_cmd)) => {
            if let Some(key) = cli_config_cmd.nexus_api_key {
                agm.set_nexus_api_key(&key);
                println!("Nexus API key set successfully.");
            } else if let Some(editor) = cli_config_cmd.editor {
                agm.set_editor(&editor);
                println!("Editor set successfully.");
            } else {
                eprintln!("Error: No configuration option specified.");
            }
        }

        Some(Command::Mod { cmd }) => match cmd {
            CliMod::Install(mut cmd) => {
                let reporter = CliInstallReporter;
                
                let profile_name = match cmd.profile.take() {
                    Some(p) => p,
                    None => {
                        let profiles = agm.get_profile_names();
                        if profiles.is_empty() {
                            eprintln!("No profiles found. Please add a profile first.");
                            return;
                        }
                        reporter.prompt_for_profile(&profiles).unwrap()
                    }
                };

                let mod_name = match cmd.name.take() {
                    Some(n) => n,
                    None => {
                        let default_name = Path::new(&cmd.files[0])
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unnamed_mod")
                            .to_string();
                        reporter.prompt_for_mod_name(&default_name).unwrap()
                    }
                };
                
                if let Err(e) = agm.install_mods(&cmd.files, &profile_name, &mod_name, &reporter).await {
                    eprintln!("Error installing mods: {}", e);
                    return;
                }

                if reporter.confirm_preset_add().unwrap() {
                    let presets = agm.get_preset_names(&profile_name);
                    if presets.is_empty() {
                        println!("No presets found for profile '{}'.", profile_name);
                        return;
                    }
                    if let Ok(selected_presets) = reporter.prompt_for_presets(&presets) {
                        if !selected_presets.is_empty() {
                            if let Err(e) = agm.add_mod_to_presets(&profile_name, &mod_name, &selected_presets) {
                                eprintln!("Error adding mod to presets: {}", e);
                            }

                            // Check if any of the selected presets are active and activate the mod if so
                            for preset in &selected_presets {
                                if agm.is_preset_active(&profile_name, preset) {
                                    println!("Activating mod '{}' as it was added to the active preset '{}'.", mod_name, preset);
                                    if let Err(e) = agm.activate_mod(&profile_name, &mod_name) {
                                        eprintln!("Error activating mod: {}", e);
                                    }
                                    break; // Activate only once
                                }
                            }
                        }
                    }
                }
            }
            CliMod::Remove { name, purge } => {
                if let Err(e) = agm.remove_mod(&name, purge) {
                    eprintln!("Error removing mod: {}", e);
                }
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