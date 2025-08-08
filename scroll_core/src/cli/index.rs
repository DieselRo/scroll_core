use crate::archive::index::{read_index, write_index, ArchiveIndex};
use std::path::Path;

pub fn index_list(archive_dir: &Path) -> Result<(), String> {
    let index_path = archive_dir.join("scroll_index.yaml");
    let idx = read_index(&index_path)?;
    println!(
        "{}",
        serde_yaml::to_string(&idx.archive_index).unwrap_or_default()
    );
    Ok(())
}

pub fn index_add(archive_dir: &Path, file: &str) -> Result<(), String> {
    let index_path = archive_dir.join("scroll_index.yaml");
    let mut idx = read_index(&index_path).unwrap_or(ArchiveIndex {
        archive_index: serde_yaml::Value::Mapping(serde_yaml::Mapping::new()),
    });
    // Basic file existence check
    let candidate = archive_dir.join(file);
    if !candidate.exists() {
        return Err(format!("File '{}' not found under archive.", file));
    }

    let ensure_list = |section: &str| -> Vec<serde_yaml::Value> {
        if let Some(root) = idx.archive_index.as_mapping() {
            if let Some(ai) = root.get(serde_yaml::Value::from("archive_index")) {
                if let Some(map) = ai.as_mapping() {
                    if let Some(seq) = map
                        .get(serde_yaml::Value::from(section))
                        .and_then(|v| v.as_sequence())
                    {
                        return seq.clone();
                    }
                }
            }
        }
        Vec::new()
    };

    let mut core = ensure_list("core_scrolls");
    let entry = serde_yaml::Value::Mapping({
        let mut m = serde_yaml::Mapping::new();
        m.insert(
            serde_yaml::Value::from("file"),
            serde_yaml::Value::from(file),
        );
        m
    });
    if !core
        .iter()
        .any(|v| v.get(serde_yaml::Value::from("file")).and_then(|f| f.as_str()) == Some(file))
    {
        core.push(entry);
    }

    // Rebuild archive_index mapping with updated core_scrolls
    let mut root = idx.archive_index.as_mapping().cloned().unwrap_or_default();
    let mut ai_map = root
        .get(serde_yaml::Value::from("archive_index"))
        .and_then(|v| v.as_mapping())
        .cloned()
        .unwrap_or_default();
    ai_map.insert(
        serde_yaml::Value::from("core_scrolls"),
        serde_yaml::Value::Sequence(core),
    );
    root.insert(
        serde_yaml::Value::from("archive_index"),
        serde_yaml::Value::Mapping(ai_map),
    );
    idx.archive_index = serde_yaml::Value::Mapping(root);

    write_index(&index_path, &idx)
}

pub fn index_remove(archive_dir: &Path, file: &str) -> Result<(), String> {
    let index_path = archive_dir.join("scroll_index.yaml");
    let mut idx = read_index(&index_path)?;
    let mut root = idx.archive_index.as_mapping().cloned().unwrap_or_default();
    let mut ai_map = root
        .get(serde_yaml::Value::from("archive_index"))
        .and_then(|v| v.as_mapping())
        .cloned()
        .unwrap_or_default();
    if let Some(seq) = ai_map
        .get_mut(serde_yaml::Value::from("core_scrolls"))
        .and_then(|v| v.as_sequence_mut())
    {
        seq.retain(|v| v.get(serde_yaml::Value::from("file")).and_then(|f| f.as_str()) != Some(file));
    }
    root.insert(
        serde_yaml::Value::from("archive_index"),
        serde_yaml::Value::Mapping(ai_map),
    );
    idx.archive_index = serde_yaml::Value::Mapping(root);
    write_index(&index_path, &idx)
}
