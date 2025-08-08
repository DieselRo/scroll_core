use scroll_core::archive::archive_loader::load_scrolls_from_directory;

#[test]
fn test_loader_handles_empty_dir() {
    let dir = tempfile::tempdir().unwrap();
    let res = load_scrolls_from_directory(dir.path());
    assert!(res.is_ok());
}


