use thiserror::Error;

use crate::parser::{FileParser, ParseError};

#[derive(Error, Debug)]
pub enum HeicError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Meta;
pub struct Parser;

impl FileParser for Parser {
    type Output = Meta;

    fn parse(_path: &std::path::Path) -> Result<Self::Output, ParseError> {
        todo!("HEIC parser")
    }
}
