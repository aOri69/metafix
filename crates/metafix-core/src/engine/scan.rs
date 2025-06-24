use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub has_json: bool,
    pub size: u64,
}

#[derive(Debug, Default)]
pub struct ScanStats {
    pub total: usize,
    pub images: usize,
    pub videos: usize,
    pub orphan_json: usize,
    pub bytes: u64,
}

#[derive(Debug, Default)]
pub struct ScanReport {
    pub files: Vec<FileEntry>,
    pub orphan_json: Vec<PathBuf>,
    pub stats: ScanStats,
    pub warnings: Vec<String>,
}

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unsupported file: {0}")]
    Unsupported(PathBuf),
}

pub fn scan<P>(path: P) -> Result<ScanReport, ScanError>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    scan_inner(path)
}

fn scan_inner(path: &Path) -> Result<ScanReport, ScanError> {
    println!("scanning from core");
    println!("{:?}", path);
    Ok(ScanReport::default())
}
