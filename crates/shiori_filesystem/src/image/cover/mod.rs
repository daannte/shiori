use std::{
    env,
    path::{Path, PathBuf},
};

use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt},
};

pub async fn get_cover(path: &Path) -> io::Result<Vec<u8>> {
    let bytes = fs::read(path).await?;
    Ok(bytes)
}

// TODO: Make an error type for this
pub async fn download_cover(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let covers_dir = env::var("COVERS_DIR").expect("COVERS_DIR must be set");
    let bytes = reqwest::get(url).await?.error_for_status()?.bytes().await?;

    let filename = url.split('/').next_back().unwrap();
    let path = PathBuf::from(covers_dir).join(filename);

    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;

    Ok(path.to_string_lossy().to_string())
}

pub async fn delete_cover(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    match fs::remove_file(path).await {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
