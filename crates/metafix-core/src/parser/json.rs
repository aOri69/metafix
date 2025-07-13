use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::parser::ParseError;

#[derive(Error, Debug)]
pub enum JsonError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TakeoutTime {
    timestamp: String,
    formatted: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakeoutGeo {
    latitude: f64,
    longitude: f64,
    altitude: f64,
    latitude_span: f64,
    longitude_span: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GoogleTakeoutJson {
    title: String,
    creation_time: TakeoutTime,
    photo_taken_time: TakeoutTime,
    geo_data: TakeoutGeo,
    geo_data_exif: TakeoutGeo,
}

pub struct JsonParser;

impl super::FileParser for JsonParser {
    type Output = GoogleTakeoutJson;

    fn parse(path: &std::path::Path) -> Result<Self::Output, ParseError> {
        let file = File::open(path).map_err(JsonError::Io)?;
        let reader = BufReader::new(file);
        let r: GoogleTakeoutJson = serde_json::from_reader(reader).map_err(JsonError::Serde)?;
        // dbg!(&r);
        Ok(r)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::parser::FileParser;
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    const TEST_JSON: &str = r#"
{
  "title": "IMG_1678.HEIC",
  "description": "",
  "imageViews": "0",
  "creationTime": {
    "timestamp": "1747494995",
    "formatted": "17 May 2025, 15:16:35 UTC"
  },
  "photoTakenTime": {
    "timestamp": "1738950318",
    "formatted": "7 Feb 2025, 17:45:18 UTC"
  },
  "geoData": {
    "latitude": 31.690999999999995,
    "longitude": -0.41769999999999996,
    "altitude": 104.7,
    "latitudeSpan": 0.0,
    "longitudeSpan": 0.0
  },
  "geoDataExif": {
    "latitude": 31.690999999999995,
    "longitude": -0.41769999999999996,
    "altitude": 104.7,
    "latitudeSpan": 0.0,
    "longitudeSpan": 0.0
  },
  "url": "https://photos.google.com/photo/verycomplicatedurl",
  "googlePhotosOrigin": {
    "mobileUpload": {
      "deviceType": "IOS_PHONE"
    }
  }
}
"#;

    /// Helper: write JSON content to a file and return its path.
    fn write_json_file(content: &str) -> (std::path::PathBuf, tempfile::TempDir) {
        let dir = tempdir().expect("create temp dir");
        let file_path = dir.path().join("test.json");
        let mut file = File::create(&file_path).expect("create file");
        file.write_all(content.as_bytes()).expect("write json");
        // tempdir will be kept alive as long as file_path is used in this scope
        (file_path, dir)
    }

    /// Approximation for geodata
    /// Epsilon ~0.11 m
    const GEO_EPS: f64 = 1e-6;

    /// # Epsilon check
    /// This function compares two [f64] numbers
    /// and returns aproximate equal result
    fn aprox_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn parses_valid_json_file() {
        const DEMO_LATITUDE: f64 = 31.690_999_999_999_995_f64;
        const DEMO_LONGITUDE: f64 = -0.417_699_999_999_999_96_f64;
        let file_path = write_json_file(TEST_JSON);

        let result = JsonParser::parse(&file_path.0);
        drop(file_path.1);
        let parsed = result.unwrap();
        let result: GoogleTakeoutJson = parsed;

        assert_eq!(result.title, "IMG_1678.HEIC");
        assert_eq!(result.creation_time.timestamp, "1747494995");
        assert_eq!(result.photo_taken_time.timestamp, "1738950318");

        assert!(aprox_eq(result.geo_data.latitude, DEMO_LATITUDE, GEO_EPS,));
        assert!(aprox_eq(result.geo_data.longitude, DEMO_LONGITUDE, GEO_EPS,));
        assert!(aprox_eq(result.geo_data.altitude, 104.7_f64, GEO_EPS));
        assert!(aprox_eq(result.geo_data.latitude_span, 0.0_f64, GEO_EPS));
        assert!(aprox_eq(result.geo_data.longitude_span, 0.0_f64, GEO_EPS));

        assert!(aprox_eq(
            result.geo_data_exif.latitude,
            DEMO_LATITUDE,
            GEO_EPS,
        ));
        assert!(aprox_eq(
            result.geo_data_exif.longitude,
            DEMO_LONGITUDE,
            GEO_EPS,
        ));
        assert!(aprox_eq(result.geo_data_exif.altitude, 104.7_f64, GEO_EPS));
        assert!(aprox_eq(
            result.geo_data_exif.latitude_span,
            0.0_f64,
            GEO_EPS
        ));
        assert!(aprox_eq(
            result.geo_data_exif.longitude_span,
            0.0_f64,
            GEO_EPS
        ));
    }

    #[test]
    fn returns_error_on_invalid_json() {
        let file_path = write_json_file("{ this is not valid json }");

        let result = JsonParser::parse(&file_path.0);
        drop(file_path.1);

        assert!(result.is_err());
    }

    #[test]
    fn returns_error_on_missing_file() {
        let result = JsonParser::parse(Path::new("nonexistent_file.json"));

        assert!(result.is_err());
    }
}
