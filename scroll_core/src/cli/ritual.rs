use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

use crate::parser::parse_scroll_from_file;
use crate::scroll_writer::ScrollWriter;
use crate::schema::ScrollStatus;
use crate::validator::{validate_scroll, validate_write_allowed};

fn ensure_under_archive(archive_dir: &Path, file: &str) -> Result<PathBuf> {
    let p = archive_dir.join(file);
    if !p.exists() {
        return Err(anyhow!(format!("File not found: {}", p.display())));
    }
    Ok(p)
}

pub fn ritual_validate(archive_dir: &Path, file: &str) -> Result<()> {
    let full = ensure_under_archive(archive_dir, file)?;
    let scroll = parse_scroll_from_file(&full)?;
    validate_scroll(&scroll.yaml_metadata).map_err(anyhow::Error::msg)?;
    println!("Validation OK: {}", file);
    Ok(())
}

pub fn ritual_validate_all(archive_dir: &Path) -> Result<()> {
    let mut ok = 0usize;
    let mut bad = 0usize;
    for entry in std::fs::read_dir(archive_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            match parse_scroll_from_file(&path) {
                Ok(scroll) => {
                    if let Err(e) = validate_scroll(&scroll.yaml_metadata) {
                        eprintln!("{}: invalid – {}", path.display(), e);
                        bad += 1;
                    } else {
                        ok += 1;
                    }
                }
                Err(e) => {
                    eprintln!("{}: parse error – {}", path.display(), e);
                    bad += 1;
                }
            }
        }
    }
    println!("Validated. OK: {}, Errors: {}", ok, bad);
    if bad > 0 { return Err(anyhow!("Validation errors present")); }
    Ok(())
}

pub fn ritual_write(archive_dir: &Path, file: &str, update_index: bool) -> Result<()> {
    let full = archive_dir.join(file);
    let scroll = parse_scroll_from_file(&full)?;
    validate_write_allowed(&scroll).map_err(anyhow::Error::msg)?;
    validate_scroll(&scroll.yaml_metadata).map_err(anyhow::Error::msg)?;
    ScrollWriter::write_scroll(&scroll, &full).map_err(anyhow::Error::msg)?;
    println!("Wrote {}", full.display());
    if update_index {
        if let Err(e) = crate::cli::index::index_add(archive_dir, file) {
            eprintln!("warning: failed to update index for '{}': {}", file, e);
        } else {
            println!("Index updated for {}", file);
        }
    }
    Ok(())
}

pub fn ritual_seal(archive_dir: &Path, file: &str) -> Result<()> {
    let full = ensure_under_archive(archive_dir, file)?;
    let mut scroll = parse_scroll_from_file(&full)?;
    if scroll.status == ScrollStatus::Sealed {
        println!("Already sealed: {}", file);
        return Ok(());
    }
    ScrollWriter::seal_scroll(&mut scroll).map_err(anyhow::Error::msg)?;
    // persist sealed state back to file
    ScrollWriter::write_scroll(&scroll, &full).map_err(anyhow::Error::msg)?;
    println!("Sealed {}", file);
    Ok(())
}


