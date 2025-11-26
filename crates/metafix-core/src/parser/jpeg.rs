use thiserror::Error;

use crate::parser::{
    FileParser, ParseError,
    exif::{ExifSegment, error::ExifError},
};

#[derive(Error, Debug)]
pub enum JpegError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Exif(#[from] ExifError),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug)]
pub struct Meta;
pub struct Parser;

impl FileParser for Parser {
    type Output = Meta;

    fn parse(path: &std::path::Path) -> Result<Self::Output, ParseError> {
        // todo!("JPEG parser")
        let file_data = std::fs::read(path).map_err(JpegError::Io)?;
        dbg!(path.display());

        let exif = ExifSegment::try_from(file_data.as_slice()).map_err(JpegError::Exif)?;
        dbg!(exif);

        Err(ParseError::WrongType(path.to_path_buf()))
    }
}
