//! Reader/writer for scroll_index.yaml governing visible archive files.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveIndex {
    pub archive_index: serde_yaml::Value,
}

pub fn read_index<P: AsRef<Path>>(path: P) -> Result<ArchiveIndex, String> {
    let raw = fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
    let val: serde_yaml::Value = serde_yaml::from_str(&raw).map_err(|e| e.to_string())?;
    Ok(ArchiveIndex { archive_index: val })
}

pub fn write_index<P: AsRef<Path>>(path: P, idx: &ArchiveIndex) -> Result<(), String> {
    let s = serde_yaml::to_string(&idx.archive_index).map_err(|e| e.to_string())?;
    fs::write(path.as_ref(), s).map_err(|e| e.to_string())
}
