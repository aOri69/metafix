use std::path::Path;
use thiserror::Error;

pub mod json;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    Json(#[from] json::JsonError),
}

pub trait FileParser {
    type Output;

    fn parse(path: &Path) -> Result<Self::Output, ParseError>;
}
