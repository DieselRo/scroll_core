use scroll_core::archive::index::{read_index, write_index, ArchiveIndex};
use std::path::Path;

#[test]
fn index_round_trip_minimal() {
    let dir = tempfile::tempdir().unwrap();
    let idx_path = dir.path().join("scroll_index.yaml");
    let init = ArchiveIndex { archive_index: serde_yaml::Value::Mapping(serde_yaml::Mapping::new()) };
    write_index(&idx_path, &init).unwrap();
    let loaded = read_index(&idx_path).unwrap();
    assert!(loaded.archive_index.is_mapping());
}


