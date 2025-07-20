use pretty_assertions::assert_eq;

#[test]
fn scan_all_json() -> Result<(), metafix_core::Error> {
    // arrange
    let tmp = metafix_test_fixtures::get_dir_with_fixtures("simple_album").unwrap();
    let root = tmp.path();
    // act
    let report = metafix_core::api::scan::scan(root)?;
    // assert
    assert_eq!(report.stats.total, 6);
    assert!(!report.files.iter().any(|entry| entry.json.is_err()));
    assert!(report.warnings.is_empty());

    Ok(())
}
