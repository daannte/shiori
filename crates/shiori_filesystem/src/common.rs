// TODO: Add some sort of file parts extension for file stuff
// tahts gets file name, stem and type.

use std::fs as sync_fs;
use std::path::{Path, PathBuf};

use tokio::fs;

pub async fn move_file(source: impl AsRef<Path>, dest: impl AsRef<Path>) -> std::io::Result<()> {
    fs::rename(source, dest).await
}

pub async fn delete_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    fs::remove_file(path).await
}

pub fn list_directories(
    path: impl AsRef<Path>,
    base_path: &PathBuf,
) -> std::io::Result<Vec<String>> {
    let mut directories = Vec::new();
    let entries = sync_fs::read_dir(&path)?;

    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            let dir_path = entry.path();
            let relative_path = dir_path
                .strip_prefix(base_path)
                .unwrap_or(&dir_path)
                .to_string_lossy()
                .to_string();

            directories.push(relative_path);
        }
    }

    Ok(directories)
}
