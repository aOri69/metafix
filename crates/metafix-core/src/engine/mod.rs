//! Internal workflows: not part of the public surface.
use thiserror::Error;

pub mod media;
pub mod scan;

#[derive(Error, Debug)]
pub enum EngineError {
    /// Wrapper for I/O errors.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
