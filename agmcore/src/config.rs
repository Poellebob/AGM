use std::fs;
use dirs_next::config_dir;
use std::io;

pub fn ensure_config_file() -> io::Result<()> {
    if let Some(mut program_dir) = config_dir() {
        program_dir.push("AGM");

        if !program_dir.exists() {
            fs::create_dir_all(&program_dir)?;
            println!("Created program directory: {}", program_dir.display());
        } else {
            println!("Program directory already exists: {}", program_dir.display());
        }

        let config_file = program_dir.join("config.yaml");

        if !config_file.exists() {
            fs::File::create(&config_file)?;
            println!("Created config file: {}", config_file.display());
        } else {
            println!("Config file already exists: {}", config_file.display());
        }
    }

    Ok(())
}
