use std::path::PathBuf;

pub fn scan(path: PathBuf) -> anyhow::Result<()> {
    println!("Scanning from CLI...{path:?}");
    let _scan_results = metafix_core::scan(path)?;
    // For now, we just return Ok to indicate success.
    Ok(())
}
