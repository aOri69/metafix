//! # Main error module.
//! Should have all errors combined together.
//! Exposed by `api` module

use thiserror::Error;

/// Main error structure.
///
/// A combination of all internal errors converted to the single struct.
/// This enum wraps standard I/O errors
/// and can be extended with additional error variants as needed.
/// `Display` and `Error` implementations are derived via `thiserror`.
#[derive(Error, Debug)]
pub enum Error {
    /// Wrapper for I/O errors.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
