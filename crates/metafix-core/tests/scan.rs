mod util;

use pretty_assertions::assert_eq;

#[test]
fn scan_all_json() -> Result<(), metafix_core::Error> {
    // arrange
    let tmp = util::sample_dir("simple_album");
    let root = tmp.path();
    dbg!(&root);
    // act
    let report = metafix_core::api::scan::scan(root)?;
    // assert
    assert_eq!(report.stats.total, 6);
    assert!(!report.files.iter().any(|entry| entry.json.is_err()));
    assert!(report.warnings.is_empty());

    Ok(())
}
