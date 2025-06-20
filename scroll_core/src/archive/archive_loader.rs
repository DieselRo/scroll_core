//    archive_loader.rs
//======================================

use crate::parser;
use crate::scroll::Scroll;
use std::fs;
use std::path::Path;

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown"))
        .unwrap_or(false)
}

/// Loads all scrolls from the given archive directory.
pub fn load_scrolls_from_directory<P: AsRef<Path>>(archive_path: P) -> Result<Vec<Scroll>, String> {
    let mut loaded_scrolls = Vec::new();
    let mut failed_count = 0;

    let entries = fs::read_dir(&archive_path)
        .map_err(|e| format!("Failed to read archive directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        let path = entry.path();

        if is_markdown_file(&path) {
            let raw_content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

            match parser::parse_scroll(&raw_content) {
                Ok(scroll) => loaded_scrolls.push(scroll),
                Err(e) => {
                    eprintln!("⚠️ Failed to parse scroll {}: {}", path.display(), e);
                    failed_count += 1;
                }
            }
        }
    }

    println!(
        "📚 Loaded {} scroll(s) from the Archive.",
        loaded_scrolls.len()
    );

    if failed_count == 0 {
        println!("🌙 All scrolls passed the veil without harm.");
    }

    Ok(loaded_scrolls)
}
