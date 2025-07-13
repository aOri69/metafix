use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{
    ScanReport,
    engine::{EngineError, media::parse_media_file},
    parser::{FileParser, ParseError, json::JsonParser},
};

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Supplementary JSON not found for media `{0}`")]
    JsonNotFound(PathBuf),
    #[error(transparent)]
    Parser(#[from] ParseError),
}

/// Internal implementation, accessed only through `api::scan::scan`.
pub fn run(path: &Path) -> Result<ScanReport, EngineError> {
    println!("scanning from core");
    println!("{}", path.to_str().unwrap_or_default());

    let (supplementary, media): (Vec<PathBuf>, Vec<PathBuf>) =
        walk(path)?.into_iter().partition(|p| {
            p.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        });

    let lookup_file_to_json = supplementary_to_lookup(supplementary);

    // Main loop with parsing
    let mut result = ScanReport::new();
    for media_file in media {
        // JSON parser was separated from media
        let json = lookup_file_to_json
            .get(&media_file)
            .ok_or(ScanError::JsonNotFound(media_file.clone()))
            .and_then(|p| JsonParser::parse(p).map_err(ScanError::Parser))
            .map_err(|e| crate::api::Error::Engine(EngineError::Scan(e)));
        // Metadata getters
        let metadata = parse_media_file(&media_file)
            .map_err(|e| crate::api::Error::Engine(EngineError::Scan(ScanError::Parser(e))));
        result.add_file(media_file, metadata, json);
    }
    Ok(result)
}

fn supplementary_to_lookup(v: Vec<PathBuf>) -> HashMap<PathBuf, PathBuf> {
    let mut result = HashMap::new();

    for p in v {
        if let Some(changed_value) = supplementary_to_filename(&p) {
            result.insert(changed_value, p);
        }
    }

    result
}

fn supplementary_to_filename(p: &Path) -> Option<PathBuf> {
    let file_name = p.file_name()?.to_str()?;
    let stripped_file_name = file_name.strip_suffix(".supplemental-metadata.json")?;
    Some(p.with_file_name(stripped_file_name))
}

fn walk<P: AsRef<Path>>(root: P) -> std::io::Result<Vec<PathBuf>> {
    // Invalid dir/file check
    std::fs::metadata(root.as_ref())?;

    let mut stack = vec![root.as_ref().to_path_buf()];
    let mut result = Vec::new();

    while let Some(entry) = stack.pop() {
        if entry.is_file() {
            result.push(entry);
        } else if entry.is_dir() {
            for child in std::fs::read_dir(&entry)? {
                stack.push(child?.path());
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::todo)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::{fs, io::Write};
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

    /// helper: create file, return its `PathBuf`
    fn touch<P: AsRef<Path>>(p: P) -> PathBuf {
        fs::File::create(&p).unwrap();
        p.as_ref().to_path_buf()
    }

    #[test]
    fn count() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let root = dir.path();
        fs::create_dir(root.join("sub_dir"))?;
        fs::File::create(root.join("sub_dir").join("photo1.jpg"))?;
        let mut f = fs::File::create(
            root.join("sub_dir")
                .join("photo1.jpg.supplemental-metadata.json"),
        )?;
        f.write_all(TEST_JSON.as_bytes())?;
        fs::File::create(root.join("sub_dir").join("photo2.tiff"))?;
        fs::File::create(root.join("sub_dir").join("photo3.heic"))?;
        fs::File::create(root.join("photo4.jpg"))?;
        let mut f = fs::File::create(root.join("photo4.jpg.supplemental-metadata.json"))?;
        f.write_all(TEST_JSON.as_bytes())?;
        fs::File::create(root.join("photo5.tiff"))?;
        let mut f = fs::File::create(root.join("photo5.tiff.supplemental-metadata.json"))?;
        f.write_all(TEST_JSON.as_bytes())?;
        fs::File::create(root.join("photo6.heic"))?;

        let report = run(root)?;

        // assert_eq!(report.stats.images, 6);
        assert_eq!(report.files.len(), 6);

        Ok(())
    }

    #[test]
    fn walk_returns_all_files_recursively() -> std::io::Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        // root files
        let f1 = touch(root.join("a.jpg"));
        let f2 = touch(root.join("b.mp4"));

        // nested dir
        let sub = root.join("nested");
        fs::create_dir(&sub)?;
        let f3 = touch(sub.join("c.json"));

        let mut files = walk(root)?
            .iter()
            .map(|p| p.strip_prefix(root).unwrap().to_owned())
            .collect::<Vec<_>>();
        files.sort();

        let mut expected = vec![
            f1.strip_prefix(root).unwrap().to_owned(),
            f2.strip_prefix(root).unwrap().to_owned(),
            f3.strip_prefix(root).unwrap().to_owned(),
        ];
        expected.sort();

        assert_eq!(files, expected);
        Ok(())
    }

    #[test]
    fn walk_empty_dir_returns_empty_vec() -> std::io::Result<()> {
        let dir = tempdir()?;
        let list = walk(dir.path())?;
        assert!(list.is_empty());
        Ok(())
    }

    #[test]
    fn walk_non_existing_path_propagates_error() {
        // println!("This is stdout");
        // eprintln!("This is stderr");
        let err = walk("/path/does/not/exist").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}
