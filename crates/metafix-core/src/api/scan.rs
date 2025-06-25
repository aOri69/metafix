use std::path::PathBuf;

use crate::{Error, engine};

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

pub fn scan<P>(root: P) -> Result<ScanReport, Error>
where
    P: AsRef<std::path::Path>,
{
    engine::scan::run(root.as_ref())
}
