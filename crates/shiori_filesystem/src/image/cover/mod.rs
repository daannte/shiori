use std::path::Path;

use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt},
};

use crate::FilesystemError;

pub async fn get_cover(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path).await
}

pub async fn download_cover(
    url: &str,
    base_path: &Path,
    media_id: &i32,
) -> Result<String, FilesystemError> {
    let covers_dir = base_path.join("covers");
    let bytes = reqwest::get(url).await?.error_for_status()?.bytes().await?;

    let filename = url.split('/').next_back().ok_or_else(|| {
        FilesystemError::Unexpected(format!("Failed to parse filename from URL: {}", url))
    })?;

    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| {
            tracing::error!(%url, "Expected file to have an extension");
            FilesystemError::InvalidExtension(url.to_string())
        })?;

    let path = covers_dir.join(format!("{media_id}.{ext}"));

    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;

    tracing::info!("Successfully saved cover to {}", path.display());

    Ok(path.to_string_lossy().to_string())
}
