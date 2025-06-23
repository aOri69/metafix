use std::path::PathBuf;

pub fn scan(path: PathBuf) -> anyhow::Result<()> {
    println!("Scanning...{path:?}");

    // For now, we just return Ok to indicate success.
    Ok(())
}

pub fn preview(path: PathBuf) -> anyhow::Result<()> {
    println!("Previewing...{path:?}");

    // For now, we just return Ok to indicate success.
    Ok(())
}

pub fn apply(path: PathBuf) -> anyhow::Result<()> {
    println!("Applying...{path:?}");

    // For now, we just return Ok to indicate success.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_placeholder() {
        scan(".".into()).expect("Scan failed");
        preview(".".into()).expect("Preview failed");
        apply(".".into()).expect("Apply failed");
    }
}
