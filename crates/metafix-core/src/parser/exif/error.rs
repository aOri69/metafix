use thiserror::Error;

use crate::parser::exif::ByteOrderError;

#[derive(Error, Debug)]
pub enum ExifError {
    #[error("{0} Segment marker not found")]
    MarkerNotFound(u8),
    #[error("Exif\\0\\0 not found")]
    ExifStartNotFound,
    #[error(transparent)]
    Endian(#[from] ByteOrderError),
    #[error("Wrong TIFF signature: {0:02X} ")]
    WrongSignature(u16),
    #[error("Unknown IFD tag: {0}")]
    UnknownIfdTag(u16),
    #[error("Cannot read slice index: {0}")]
    SliceRead(&'static str),
    #[error(transparent)]
    TryFromSlice(#[from] std::array::TryFromSliceError),
}
