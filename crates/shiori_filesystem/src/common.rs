// TODO: Add some sort of file parts extension for file stuff
// tahts gets file name, stem and type.

use std::path::Path;

use tokio::fs;

pub async fn move_file(source: &Path, dest: &Path) -> std::io::Result<()> {
    fs::rename(source, dest).await
}
