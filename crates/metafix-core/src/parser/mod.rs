use std::path::{Path, PathBuf};
use thiserror::Error;

pub mod heic;
pub mod jpeg;
pub mod json;
pub mod tiff;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    Json(#[from] json::JsonError),
    #[error(transparent)]
    Jpeg(#[from] jpeg::JpegError),
    #[error(transparent)]
    Heic(#[from] heic::HeicError),
    #[error(transparent)]
    Tiff(#[from] tiff::TiffError),
    #[error("Wrong file type passed to the parser `{0}`")]
    WrongType(PathBuf),
}

pub trait FileParser {
    type Output;

    fn parse(path: &Path) -> Result<Self::Output, ParseError>;
}

#[derive(Debug)]
pub enum MediaMeta {
    Jpeg(jpeg::Meta),
    Tiff(tiff::Meta),
    Heic(heic::Meta),
}
