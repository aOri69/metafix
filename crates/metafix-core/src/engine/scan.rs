use std::path::{Path, PathBuf};

use crate::{Error, ScanReport};

/// Internal implementation, accessed only through `api::scan::scan`.
pub(crate) fn run(path: &Path) -> Result<ScanReport, Error> {
    println!("scanning from core");
    println!("{path:?}");
    let _files = walk(path)?;
    Ok(ScanReport::default())
}

fn walk<P: AsRef<Path>>(root: P) -> std::io::Result<Vec<PathBuf>> {
    let mut stack = vec![root.as_ref().to_path_buf()];
    let mut result = Vec::new();

    //TODO Add invalid dir check?

    while let Some(entry) = stack.pop() {
        if entry.is_file() {
            result.push(entry);
        } else if entry.is_dir() {
            for child in std::fs::read_dir(&entry)? {
                stack.push(child?.path());
            }
        } else {
            continue;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::tempdir;

    /// helper: create file, return its PathBuf
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
        fs::File::create(root.join("photo1.jpg.supplemental-metadata.json"))?;
        fs::File::create(root.join("sub_dir").join("photo2.tiff"))?;
        fs::File::create(root.join("sub_dir").join("photo3.heic"))?;
        fs::File::create(root.join("photo4.jpg"))?;
        fs::File::create(root.join("photo5.tiff"))?;
        fs::File::create(root.join("photo5.tiff.supplemental-metadata.json"))?;
        fs::File::create(root.join("photo6.heic"))?;
        fs::File::create(root.join("photo4.jpg.supplemental-metadata.json"))?;

        let report = run(root)?;

        assert_eq!(report.stats.images, 6);
        assert_eq!(report.files.len(), 2);

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
        let err = walk("/path/does/not/exist").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}
