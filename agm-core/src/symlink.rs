use std::fs;
use std::io;
use std::path::Path;

#[cfg(unix)]
pub fn create_symlink(source: &Path, destination: &Path) -> io::Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    std::os::unix::fs::symlink(source, destination)
}

#[cfg(windows)]
pub fn create_symlink(source: &Path, destination: &Path) -> io::Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    if source.is_dir() {
        std::os::windows::fs::symlink_dir(source, destination)
    } else {
        std::os::windows::fs::symlink_file(source, destination)
    }
}

#[cfg(not(any(unix, windows)))]
pub fn create_symlink(source: &Path, destination: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Symlinks are not supported on this platform.",
    ))
}
