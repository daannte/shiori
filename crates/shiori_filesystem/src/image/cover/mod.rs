use std::path::Path;

use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt},
};

pub async fn get_cover(path: &Path) -> io::Result<Vec<u8>> {
    let bytes = fs::read(path).await?;
    Ok(bytes)
}

// TODO: Make an error type for this
pub async fn download_cover(
    url: &str,
    base_path: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let covers_dir = base_path.join("covers");
    let bytes = reqwest::get(url).await?.error_for_status()?.bytes().await?;

    let filename = url.split('/').next_back().unwrap();
    let path = covers_dir.join(filename);

    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;

    Ok(path.to_string_lossy().to_string())
}
