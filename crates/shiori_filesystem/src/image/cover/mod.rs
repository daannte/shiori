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
    media_id: &i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let covers_dir = base_path.join("covers");
    let bytes = reqwest::get(url).await?.error_for_status()?.bytes().await?;

    let filename = url.split('/').next_back().unwrap();
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .ok_or("Expected file to have an extension")?;

    let path = covers_dir.join(format!("{media_id}.{ext}"));

    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;

    Ok(path.to_string_lossy().to_string())
}
