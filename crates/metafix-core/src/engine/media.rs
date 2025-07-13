use std::path::Path;

use crate::parser::{FileParser, MediaMeta, ParseError, heic, jpeg};

pub enum MediaType {
    Jpeg,
    Heic,
}

pub fn parse_media_file(path: &Path) -> Result<MediaMeta, ParseError> {
    Ok(match detect_media_type(path) {
        Some(MediaType::Jpeg) => MediaMeta::Jpeg(jpeg::Parser::parse(path)?),
        Some(MediaType::Heic) => MediaMeta::Heic(heic::Parser::parse(path)?),
        None => {
            return Err(ParseError::WrongType(path.to_path_buf()));
        }
    })
}

fn detect_media_type(path: &std::path::Path) -> Option<MediaType> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)?;
    match extension.as_str() {
        "jpg" | "jpeg" => Some(MediaType::Jpeg),
        "heic" => Some(MediaType::Heic),
        _ => None,
    }
}
