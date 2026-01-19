use crate::config::Config;
use crate::mod_spec::{FileEntry, ModSpec};
use crate::profile::Profile;
use std::fs;
use std::io::{self};
use std::path::Path;
use walkdir::WalkDir;
use zip::ZipArchive;

pub trait InstallReporter {
    fn unpacking_start(&self, file_name: &str, dest: &str);
    fn review_placements(&self, mod_name: &str);
    fn prompt_for_point(&self, target: &str, moddirs: &[String]) -> io::Result<String>;
}

pub async fn install_mods(
    files: &[String],
    profile_name: &str,
    reporter: &dyn InstallReporter,
) -> io::Result<()> {
    let data_dir = Config::get_data_dir();
    let profile_path = data_dir.join("profiles").join(format!("{}.yaml", profile_name));

    if !profile_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Profile '{}' not found at {}", profile_name, profile_path.display()),
        ));
    }

    let profile = Profile::from_file(&profile_path);

    for file_path_str in files {
        let file_path = Path::new(file_path_str);
        if let Some(extension) = file_path.extension() {
            if extension == "zip" {
                let mod_name = file_path.file_stem().unwrap().to_str().unwrap().to_string();
                let storage_path = data_dir.join("storage").join(&profile.game.name).join(&mod_name);

                if !storage_path.exists() {
                    fs::create_dir_all(&storage_path)?;
                }

                reporter.unpacking_start(file_path.to_str().unwrap(), storage_path.to_str().unwrap());

                let file = fs::File::open(&file_path)?;
                let mut archive = ZipArchive::new(file)?;

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let outpath = match file.enclosed_name() {
                        Some(path) => storage_path.join(path),
                        None => continue,
                    };

                    if (*file.name()).ends_with('/') {
                        fs::create_dir_all(&outpath)?;
                    } else {
                        if let Some(p) = outpath.parent() {
                            if !p.exists() {
                                fs::create_dir_all(&p)?;
                            }
                        }
                        let mut outfile = fs::File::create(&outpath)?;
                        io::copy(&mut file, &mut outfile)?;
                    }
                }

                let sidecar_filename = format!("{}.yaml", file_path.file_stem().unwrap().to_str().unwrap());
                let sidecar_path_in_storage = storage_path.join(&sidecar_filename);

                let mut files_entries = Vec::new();
                for entry in WalkDir::new(&storage_path).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        let current_file_path = entry.path();
                        if current_file_path == sidecar_path_in_storage {
                            continue;
                        }

                        let target = current_file_path.strip_prefix(&storage_path).unwrap().to_str().unwrap().to_string();
                        let point = if let Some(ext) = current_file_path.extension().and_then(|s| s.to_str()) {
                            profile.layout.iter()
                                .find_map(|layout_node| layout_node.find_matching_moddir_point(ext))
                                .unwrap_or("".to_string())
                        } else {
                            "".to_string()
                        };
                        files_entries.push(FileEntry { target, point });
                    }
                }

                let mut mod_spec = ModSpec {
                    name: mod_name.clone(),
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
                let yaml_string = serde_yaml::to_string(&mod_spec).unwrap();
                fs::write(sidecar_path, yaml_string)?;
            }
        }
    }

    Ok(())
}
