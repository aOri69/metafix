//! Scanning module: walks directories and collects media file info for reporting.
//!
//! This module provides the core logic for traversing a directory tree, detecting supported
//! media files (images, videos), and associating supplementary JSON files (e.g., from Google Takeout).
//! It produces a structured [`ScanReport`] containing file entries, statistics, orphaned JSONs,
//! and any warnings encountered during traversal.
//!
//! # Example
//!
//! ```
//! //todo!("Fix this doctest")
//! //use metafix_core::scan;
//! //# fn main() -> Result<(), metafix_core::Error> {
//! //let report = scan("./test_photos")?;
//! //println!("Found {} files", report.stats.total);
//! //# Ok(())
//! //# }
//! ```

use std::path::PathBuf;

use crate::{
    Error, engine,
    parser::{MediaMeta, json::GoogleTakeoutJson},
};

/// Information about a single media file discovered during scanning.
///
/// Each entry records:
/// - The absolute path to the media file.
/// - The result of parsing its metadata (`Ok(MediaMeta)` or `Err(ParseError)`).
/// - Optionally, associated Google Takeout JSON metadata.
///
/// This design allows the scan to capture both successes and failures for each file,
/// enabling detailed diagnostics and reporting.
#[derive(Debug)]
pub struct FileEntry {
    /// Absolute path to the media file.
    pub path: PathBuf,
    /// Metadata from the mediafile
    pub meta: Result<MediaMeta, Error>,
    /// Whether a related JSON metadata file was found.
    pub json: Result<GoogleTakeoutJson, Error>,
}
/// Aggregated statistics about the scan operation.
///
/// Tracks:
/// - Total number of media files found.
///
#[derive(Debug, Default)]
pub struct ScanStats {
    /// Total number of media files found.
    pub total: usize,
}

/// The complete result of a directory scan.
///
/// Contains:
/// - All discovered media files and their parsing results.
/// - Aggregated statistics.
/// - Any warnings or non-fatal issues encountered (e.g., unreadable files, unexpected formats).
#[derive(Debug, Default)]
pub struct ScanReport {
    /// List of all media files found.
    pub files: Vec<FileEntry>,
    /// Aggregated statistics for the scan.
    pub stats: ScanStats,
    /// Any warnings or non-fatal errors encountered.
    pub warnings: Vec<String>,
}

impl ScanReport {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds a new media file entry to the scan report.
    ///
    /// This method appends a [`FileEntry`] to the report's file list, incrementing the total file count.
    /// It records the file's path, the result of metadata parsing, and optionally associated Google Takeout JSON metadata.
    ///
    /// # Parameters
    ///
    /// - `path`: Absolute or canonical path to the media file.
    /// - `meta`: The result of parsing the file's media metadata.
    ///   Contains either parsed metadata (`Ok(MediaMeta)`) or a parsing error (`Err(ParseError)`).
    /// - `json`: Optional Google Takeout JSON metadata associated with this file, if found.
    ///
    /// After calling this method, the file entry is available in [`ScanReport::files`], and the total file count is incremented.
    pub(crate) fn add_file(
        &mut self,
        path: PathBuf,
        meta: Result<MediaMeta, Error>,
        json: Result<GoogleTakeoutJson, Error>,
    ) {
        self.files.push(FileEntry { path, meta, json });
        self.stats.total += 1;
    }
}

/// Iteratively scans the given root directory for media and JSON files,
/// returning a [`ScanReport`] with all discovered information.
///
/// # Errors
///
/// Returns an [`Error`] if the directory cannot be read or if a fatal I/O error occurs.
/// Non-fatal errors for individual files are recorded in the `meta` field of each [`FileEntry`].
///
/// # Example
///
/// ```
/// //todo!("Write doctest")
/// ```
pub fn scan<P>(root: P) -> Result<ScanReport, Error>
where
    P: AsRef<std::path::Path>,
{
    Ok(engine::scan::run(root.as_ref())?)
}
