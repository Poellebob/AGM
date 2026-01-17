mod config;

pub fn core_logic() {
    config::ensure_config_file();
}

// profile
pub fn list_profiles() {
    println!("This is your profiles");
}

pub fn add_profile(path: String, name: String) {
    println!("Add a game profile");
}

pub fn delete_profile(game: String) {
    println!("Delete a game profile");
}

pub fn edit_profile() {
    println!("Edit a profile");
}

// preset
pub fn switch_preset(game: String, preset: String) {
    println!("Switch preset");
}

pub fn list_presets() {
    println!("Show all preset");
}
