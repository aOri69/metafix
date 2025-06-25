use std::path::Path;

use crate::{Error, ScanReport};

/// Internal implementation, accessed only through `api::scan::scan`.
pub(crate) fn run(path: &Path) -> Result<ScanReport, Error> {
    println!("scanning from core");
    println!("{:?}", path);
    Ok(ScanReport::default())
}
