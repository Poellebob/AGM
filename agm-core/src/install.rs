use crate::config::Config;
use crate::mod_spec::{FileEntry, ModSpec};
use crate::profile::Profile;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tar::Archive;
use walkdir::WalkDir;
use zip::ZipArchive;

pub trait InstallReporter {
    fn unpacking_start(&self, file_name: &str, dest: &str);
    fn review_placements(&self, mod_name: &str);
    fn prompt_for_point(&self, target: &str, moddirs: &[String]) -> io::Result<String>;
    fn symlink_created(&self, source: &Path, destination: &Path);
    fn prompt_for_unpack(&self, file_name: &str) -> io::Result<bool>;
    fn prompt_for_profile(&self, profiles: &[String]) -> io::Result<String>;
    fn prompt_for_mod_name(&self, default_name: &str) -> io::Result<String>;
    fn confirm_preset_add(&self) -> io::Result<bool>;
    fn prompt_for_presets(&self, presets: &[String]) -> io::Result<Vec<String>>;
    fn confirm_profile_parts_removal(&self) -> io::Result<(bool, bool)>;
    fn warn(&self, message: &str);
}

async fn unpack_zip(
    file_path: &Path,
    storage_path: &Path,
    reporter: &dyn InstallReporter,
) -> io::Result<()> {
    reporter.unpacking_start(file_path.to_str().unwrap(), storage_path.to_str().unwrap());
    let file = fs::File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => storage_path.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

async fn unpack_rar(
    file_path: &Path,
    storage_path: &Path,
    reporter: &dyn InstallReporter,
) -> io::Result<()> {
    reporter.unpacking_start(file_path.to_str().unwrap(), storage_path.to_str().unwrap());
    let dest_path = storage_path.join(file_path.file_name().unwrap());
    fs::copy(file_path, dest_path)?;
    reporter.warn(&format!("Warning: .rar files are not automatically unpacked yet. Please extract '{}' manually if needed.", file_path.display()));
    Ok(())
}

async fn unpack_7z(
    file_path: &Path,
    storage_path: &Path,
    reporter: &dyn InstallReporter,
) -> io::Result<()> {
    reporter.unpacking_start(file_path.to_str().unwrap(), storage_path.to_str().unwrap());
    let dest_path = storage_path.join(file_path.file_name().unwrap());
    fs::copy(file_path, dest_path)?;
    reporter.warn(&format!("Warning: .7z files are not automatically unpacked yet. Please extract '{}' manually if needed.", file_path.display()));
    Ok(())
}

async fn unpack_tar(
    file_path: &Path,
    storage_path: &Path,
    reporter: &dyn InstallReporter,
) -> io::Result<()> {
    reporter.unpacking_start(file_path.to_str().unwrap(), storage_path.to_str().unwrap());
    let file = fs::File::open(file_path)?;
    let mut archive = Archive::new(file);
    archive.unpack(storage_path)?;
    Ok(())
}

async fn handle_file(
    file_path: &Path,
    profile: &Profile,
    data_dir: &Path,
    mod_name: &str,
    reporter: &dyn InstallReporter,
) -> io::Result<()> {
    let file_name = file_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown_file")
        .to_string();

    let storage_path = data_dir.join("storage").join(&profile.game.name).join(mod_name);
    fs::create_dir_all(&storage_path)?;

    // Check if the file itself is an archive based on profile MIME types
    let file_extension = file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if reporter.prompt_for_unpack(&file_name)? {
        match file_extension {
            "zip" => unpack_zip(file_path, &storage_path, reporter).await?,
            "rar" => {
                unpack_rar(file_path, &storage_path, reporter).await?;
            },
            "7z" => unpack_7z(file_path, &storage_path, reporter).await?,
            "tar" => unpack_tar(file_path, &storage_path, reporter).await?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unsupported archive type for unpacking: {}", file_extension),
                ));
            }
        }
    } else {
        let dest_path = storage_path.join(&file_name);
        fs::copy(file_path, dest_path)?;
    }

    let sidecar_filename = format!("{}.yaml", mod_name);
    let sidecar_path_in_storage = storage_path.join(&sidecar_filename);

    let mut files_entries = Vec::new();
    for entry in WalkDir::new(&storage_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let current_file_path = entry.path();
            if current_file_path == sidecar_path_in_storage {
                continue;
            }

            let target = current_file_path
                .strip_prefix(&storage_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let point = if let Some(ext) = current_file_path.extension().and_then(|s| s.to_str()) {
                profile
                    .layout
                    .iter()
                    .find_map(|layout_node| layout_node.find_matching_moddir_point(ext))
                    .unwrap_or("".to_string())
            } else {
                "".to_string()
            };
            files_entries.push(FileEntry { target, point });
        }
    }

    let mut mod_spec = ModSpec {
        name: mod_name.to_string(),
        url: None,
        files: files_entries,
    };

    reporter.review_placements(&mod_spec.name);
    let moddir_options = profile.get_moddir_names();

    for file_entry in &mut mod_spec.files {
        if file_entry.point.is_empty() {
            let user_choice = reporter.prompt_for_point(&file_entry.target, &moddir_options)?;
            file_entry.point = user_choice;
        }
    }
    
    let sidecar_path = storage_path.join(sidecar_filename);
    let yaml_string = serde_yaml::to_string(&mod_spec)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(sidecar_path, yaml_string)?;

    Ok(())
}

pub async fn install_mods(
    files: &[String],
    profile_name: &str,
    mod_name: &str,
    reporter: &dyn InstallReporter,
) -> io::Result<()> {
    let data_dir = Config::get_data_dir()?;
    let profile_path = data_dir.join("profiles").join(format!("{}.yaml", profile_name));

    if !profile_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Profile '{}' not found at {}",
                profile_name,
                profile_path.display()
            ),
        ));
    }

    let profile = Profile::from_file(&profile_path);

    for file_path_str in files {
        let file_path = PathBuf::from(file_path_str);
        handle_file(&file_path, &profile, &data_dir, mod_name, reporter).await?;
    }

    // Add mod to config after successful installation
    let mut config = Config::load().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    config.add_mod_to_game(profile_name, mod_name);
    config.save().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}
