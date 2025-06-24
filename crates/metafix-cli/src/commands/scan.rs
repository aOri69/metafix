use std::path::PathBuf;

pub fn scan(path: PathBuf) -> anyhow::Result<()> {
    println!("Scanning from CLI...{path:?}");
    metafix_core::engine::scan(path)?;
    // For now, we just return Ok to indicate success.
    Ok(())
}
