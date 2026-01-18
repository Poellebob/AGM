use std::path::Path;

#[cfg(target_os = "linux")]
pub use std::os::unix::fs::symlink as symlink;

#[cfg(target_os = "windows")]
pub use std::os::windows::fs::symlink_file as symlink;

pub fn link_files(target_files: &[String], point_dir: &Path) {
    for target_file in target_files {
        let target_path = Path::new(target_file);
        let link_path = point_dir.join(target_path.file_name().unwrap());

        symlink(target_path, &link_path).expect("Failed to create symlink");
    }
}