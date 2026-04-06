use std::{fs, path::Path};

use rbook::Epub as Rpub;

pub struct Epub;

impl Epub {
    // TODO: Change to return Result
    pub fn get_cover_path(media_id: &i32, path: &Path, base_path: &Path) -> Option<String> {
        let covers_dir = base_path.join("covers");
        let epub = Rpub::open(path).ok()?;
        let cover = epub.manifest().cover_image()?;
        let ext = cover.href().extension()?;
        let data = cover.read_bytes().ok()?;

        let cover_path = Path::new(&covers_dir).join(format!("{media_id}.{ext}"));

        fs::write(&cover_path, data).ok()?;

        Some(cover_path.to_string_lossy().to_string())
    }
}
