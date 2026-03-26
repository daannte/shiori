use std::path::Path;

use tokio::{fs, io};

pub async fn get_cover(path: &Path) -> io::Result<Vec<u8>> {
    let bytes = fs::read(path).await?;
    Ok(bytes)
}
