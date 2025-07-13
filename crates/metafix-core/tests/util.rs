//! # Integration test utitlities
//!
//! Temporary directories via [tempfile]
//! Epsilon checks for [f64]

#![allow(clippy::expect_used)]
#![allow(clippy::unnecessary_debug_formatting)]

use std::path::Path;
use tempfile::TempDir;

#[must_use]
/// # Panics
/// - `TempDir` could not be created
/// - `fs_extra::dir::copy` failed
pub fn sample_dir(sample_name: &str) -> TempDir {
    let tmp = tempfile::tempdir().expect("Expected to create temproary dir");
    let src = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(sample_name);
    fs_extra::dir::copy(&src, &tmp, &fs_extra::dir::CopyOptions::new())
        .unwrap_or_else(|_| panic!("Expected to copy sample dir {src:?} to {tmp:?}"));
    tmp
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
