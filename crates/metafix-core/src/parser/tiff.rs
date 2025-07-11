use thiserror::Error;

use crate::parser::{FileParser, ParseError};

#[derive(Error, Debug)]
pub enum TiffError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug)]
pub struct Meta;
pub struct Parser;

impl FileParser for Parser {
    type Output = Meta;

    fn parse(path: &std::path::Path) -> Result<Self::Output, ParseError> {
        // todo!("TIFF parser")
        Err(ParseError::WrongType(path.to_path_buf()))
    }
}
