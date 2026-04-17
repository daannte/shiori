use std::{
    fs::File,
    io::{self, Read, Seek},
    path,
};

/// https://github.com/koreader/koreader/blob/master/frontend/util.lua#L1111
pub fn generate_koreader_hash<P: AsRef<path::Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut ctx = md5::Context::new();
    let step = 1024;
    let size = 1024;

    for i in -1..=10 {
        let pos = if i < 0 { 0 } else { step << (i * 2) };
        file.seek(std::io::SeekFrom::Start(pos as u64))?;

        let mut buf = vec![0u8; size];
        let bytes = file.read(&mut buf)?;

        if bytes == 0 {
            break;
        }

        ctx.consume(buf);
    }
    let hash = format!("{:x}", ctx.finalize());
    tracing::debug!(hash = %hash, "Generated koreader hash");

    Ok(hash)
}

// https://github.com/koreader/koreader/blob/master/spec/unit/util_spec.lua#L337C1-L346C1
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn get_path(path: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn test_koreader_md5_epub() {
        let path = get_path("tests/data/leaves.epub");
        let hash = generate_koreader_hash(path).unwrap();
        assert_eq!("59d481d168cca6267322f150c5f6a2a3", hash)
    }

    #[test]
    fn test_koreader_md5_pdf() {
        let path = get_path("tests/data/tall.pdf");
        let hash = generate_koreader_hash(path).unwrap();
        assert_eq!("41cce710f34e5ec21315e19c99821415", hash)
    }
}
