mod util;

use pretty_assertions::assert_eq;

#[test]
fn scan_all_json() -> Result<(), metafix_core::Error> {
    // arrange
    let tmp = util::sample_dir("good");
    let root = tmp.path();
    dbg!(&root);
    // act
    let report = metafix_core::api::scan::scan(root)?;
    dbg!(&report);
    // assert
    assert_eq!(report.stats.total, 16);
    assert!(!report.files.iter().any(|entry| entry.json.is_none()));
    assert!(report.warnings.is_empty());

    Ok(())
}
