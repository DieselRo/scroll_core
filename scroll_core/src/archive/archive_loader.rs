//! Utility functions for loading scroll files from a directory on startup.
//! The loader filters for markdown files and returns parsed Scroll structs.
//! See [ArchiveLoader](../../AGENTS.md#filereader) for related constructs.
//    archive_loader.rs
//======================================

use crate::archive::index::read_index;
use crate::parser;
use crate::scroll::Scroll;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown"))
        .unwrap_or(false)
}

/// Loads all scrolls from the given archive directory.
pub fn load_scrolls_from_directory<P: AsRef<Path>>(archive_path: P) -> Result<Vec<Scroll>, String> {
    let archive_path = archive_path.as_ref();

    fn enumerate_markdown_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
        let mut out = Vec::new();
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if entry.file_type().is_dir() {
                // Skip any "Deactivated scrolls" directories entirely
                if path.file_name().and_then(|n| n.to_str()) == Some("Deactivated scrolls") {
                    continue;
                }
            }
            if entry.file_type().is_file() && is_markdown_file(path) {
                // Also skip files under a deactivated path segment
                let path_str = path.to_string_lossy();
                if path_str.contains("Deactivated scrolls") { continue; }
                out.push(path.to_path_buf());
            }
        }
        if out.is_empty() {
            // Back-compat: top-level read to at least exercise errors
            let entries = fs::read_dir(dir)
                .map_err(|e| format!("Failed to read archive directory: {}", e))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
                let path = entry.path();
                if is_markdown_file(&path) {
                    out.push(path);
                }
            }
        }
        Ok(out)
    }

    // Build list via index if present
    let index_path = archive_path.join("scroll_index.yaml");
    let file_list: Vec<PathBuf> = if index_path.exists() {
        match read_index(&index_path) {
            Ok(idx) => {
                let mut files = Vec::new();
                if let Some(root) = idx.archive_index.as_mapping() {
                    for (k, v) in root {
                        let key = k.as_str().unwrap_or("");
                        if key == "archive_index" {
                            if let Some(ai_map) = v.as_mapping() {
                                if let Some(core) = ai_map.get(&serde_yaml::Value::from("core_scrolls")) {
                                    if let Some(list) = core.as_sequence() {
                                        for item in list {
                                            if let Some(f) = item.get(&serde_yaml::Value::from("file")) {
                                                if let Some(s) = f.as_str() {
                                                    files.push(archive_path.join(s));
                                                }
                                            }
                                        }
                                    }
                                }
                                if let Some(tech) = ai_map.get(&serde_yaml::Value::from("technical_scrolls")) {
                                    if let Some(list) = tech.as_sequence() {
                                        for item in list {
                                            if let Some(f) = item.get(&serde_yaml::Value::from("file")) {
                                                if let Some(s) = f.as_str() {
                                                    files.push(archive_path.join(s));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if files.is_empty() { enumerate_markdown_files(archive_path)? } else { files }
            }
            Err(e) => {
                eprintln!("⚠️ Failed to read scroll_index.yaml: {}. Falling back to directory scan.", e);
                enumerate_markdown_files(archive_path)?
            }
        }
    } else {
        enumerate_markdown_files(archive_path)?
    };

    let mut loaded_scrolls = Vec::new();
    let mut failed_count = 0;

    for path in file_list {
        match fs::read_to_string(&path) {
            Ok(raw_content) => match parser::parse_scroll(&raw_content) {
                Ok(scroll) => loaded_scrolls.push(scroll),
                Err(e) => {
                    eprintln!("⚠️ Failed to parse scroll {}: {}", path.display(), e);
                    failed_count += 1;
                }
            },
            Err(e) => {
                eprintln!("⚠️ Failed to read file {}: {}", path.display(), e);
                failed_count += 1;
                continue;
            }
        }
    }

    println!(
        "📚 Loaded {} scroll(s) from the Archive.",
        loaded_scrolls.len()
    );

    if failed_count == 0 {
        println!("🌙 All scrolls passed the veil without harm.");
    } else {
        println!("🌒 {} scroll(s) failed to load or parse; see warnings above.", failed_count);
    }

    Ok(loaded_scrolls)
}
