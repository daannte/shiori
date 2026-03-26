use std::{env, fs, path::Path};

use rbook::Epub as Rpub;

pub struct Epub;

impl Epub {
    // TODO: Change to return Result
    pub fn get_cover_path(file_stem: &str, path: &Path) -> Option<String> {
        // NOTE: Temp dir for now
        let covers_dir = env::var("COVERS_DIR").expect("COVERS_DIR must be set");
        let epub = Rpub::open(path).ok()?;
        let cover = epub.manifest().cover_image()?;
        let ext = cover.href().extension()?;
        let data = cover.read_bytes().ok()?;

        let cover_path = Path::new(&covers_dir).join(format!("{}.{}", file_stem, ext));

        fs::write(&cover_path, data).ok()?;

        Some(cover_path.to_string_lossy().to_string())
    }
}
