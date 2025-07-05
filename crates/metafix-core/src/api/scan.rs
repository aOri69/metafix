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
//! use metafix_core::scan::{scan, ScanReport};
//! let report: ScanReport = scan("./photos")?;
//! println!("Found {} files", report.stats.total);
//! ```

use std::path::PathBuf;

use crate::{Error, engine};

/// Information about a single media file discovered during scanning.
///
/// Each entry records the file path, whether a matching JSON was found,
/// and the file's size in bytes.
#[derive(Debug)]
pub struct FileEntry {
    /// Absolute path to the media file.
    pub path: PathBuf,
    /// Whether a related JSON metadata file was found.
    pub has_json: bool,
    /// File size in bytes.
    pub size: u64,
}
/// Aggregated statistics about the scan operation.
///
/// Tracks total files, images, videos, orphan JSONs, and total size.
#[derive(Debug, Default)]
pub struct ScanStats {
    /// Total number of media files found.
    pub total: usize,
    /// Number of image files.
    pub images: usize,
    /// Number of video files.
    pub videos: usize,
    /// Number of JSON files without a matching media file.
    pub orphan_json: usize,
    /// Total bytes of all media files.
    pub bytes: u64,
}

/// The complete result of a directory scan.
///
/// Contains a list of discovered files, orphaned JSONs,
/// aggregate statistics, and any warnings encountered.
#[derive(Debug, Default)]
pub struct ScanReport {
    /// List of all media files found.
    pub files: Vec<FileEntry>,
    /// JSON files not matched to any media file.
    pub orphan_json: Vec<PathBuf>,
    /// Aggregated statistics for the scan.
    pub stats: ScanStats,
    /// Any warnings or non-fatal errors encountered.
    pub warnings: Vec<String>,
}

/// Iteratively scans the given root directory for media and JSON files,
/// returning a [`ScanReport`] with all discovered information.
///
/// # Errors
///
/// Returns an [`Error`] if the directory cannot be read or if a fatal I/O error occurs.
///
/// # Example
///
/// ```
/// use metafix_core::scan::scan;
/// let report = scan("./photos")?;
/// println!("Total files: {}", report.stats.total);
/// # Ok::<(), metafix_core::Error>(())
/// ```
pub fn scan<P>(root: P) -> Result<ScanReport, Error>
where
    P: AsRef<std::path::Path>,
{
    Ok(engine::scan::run(root.as_ref())?)
}
