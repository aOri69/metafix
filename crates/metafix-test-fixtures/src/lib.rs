//! # Integration test utitlities
//!
//! Temporary directories via [tempfile]
//! Epsilon checks for [f64]

#![allow(clippy::expect_used)]
#![allow(clippy::unnecessary_debug_formatting)]

use std::path::Path;
use tempfile::TempDir;

/// # Panics
/// - `TempDir` could not be created
/// - `fs_extra::dir::copy` failed
pub fn get_dir_with_fixtures(fixture_name: &str) -> anyhow::Result<TempDir> {
    let tmp = tempfile::tempdir()?;
    let src = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(fixture_name);
    fs_extra::dir::copy(&src, &tmp, &fs_extra::dir::CopyOptions::new())
        .map_err(|e| anyhow::anyhow!("Failed to copy fixture dir {src:?} to {tmp:?}: {e}"))?;
    Ok(tmp)
}

/// Approximation for geodata
/// Epsilon ~0.11 m
pub const GEO_EPS: f64 = 1e-6;

/// # Epsilon check
/// This function compares two [f64] numbers
/// and returns aproximate equal result
pub fn aprox_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}
