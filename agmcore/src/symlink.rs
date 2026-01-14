#[cfg(target_os = "linux")]
pub use std::os::unix::fs::symlink as symlink;

#[cfg(target_os = "windows")]
pub use std::os::windows::fs::symlink_file as symlink;

pub fn linkFiles(target: [String], point: String) -> std::io::Result<()> {
    for i in 0..target.len() {
        symlink(target[i], point)?;
    }
    Ok(())
}