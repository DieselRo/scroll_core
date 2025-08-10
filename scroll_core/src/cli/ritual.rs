use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

use crate::parser::parse_scroll_from_file;
use crate::schema::ScrollStatus;
use crate::scroll_writer::ScrollWriter;
use crate::validator::{validate_scroll, validate_write_allowed};
use migration::MigratorTrait;
use sea_orm::DatabaseConnection;

fn ensure_db() -> DatabaseConnection {
    if crate::sessions::database::is_initialized() {
        return crate::sessions::database::get_db_connection().clone();
    }
    let raw = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://scroll_core.db".into());
    // Normalize SQLite URL: strip query, canonicalize Windows paths, ensure sqlite:/// prefix
    let mut base = match raw.find('?') { Some(idx) => raw[..idx].to_string(), None => raw };
    if base.starts_with("sqlite://") {
        let path_part = &base[9..];
        // If not already sqlite:/// and looks like Windows drive path, canonicalize
        if !base.starts_with("sqlite::///") {
            let p = std::path::Path::new(path_part);
            let abs = if p.is_absolute() { p.to_path_buf() } else { std::env::current_dir().unwrap().join(p) };
            let norm = abs.to_string_lossy().replace('\\', "/");
            base = format!("sqlite:///{}", norm);
        }
    }
    let db_url = base;
    let rt = tokio::runtime::Runtime::new().expect("rt");
    rt.block_on(async move {
        crate::sessions::database::init_sqlite_connection(&db_url)
            .await
            .expect("DB init failed");
        let conn = crate::sessions::database::get_db_connection().clone();
        migration::Migrator::up(&conn, None)
            .await
            .expect("migrations failed");
        conn
    })
}

fn ensure_under_archive(archive_dir: &Path, file: &str) -> Result<PathBuf> {
    let as_path = Path::new(file);
    if as_path.is_absolute() {
        if as_path.exists() {
            return Ok(as_path.to_path_buf());
        } else {
            return Err(anyhow!(format!("File not found: {}", as_path.display())));
        }
    }
    let joined = archive_dir.join(file);
    if !joined.exists() {
        return Err(anyhow!(format!("File not found: {}", joined.display())));
    }
    Ok(joined)
}

pub fn ritual_validate(archive_dir: &Path, file: &str) -> Result<()> {
    let full = ensure_under_archive(archive_dir, file)?;
    let scroll = parse_scroll_from_file(&full)?;
    let conn = ensure_db();
    // Use canonical path to ensure stable matching
    let scroll_path = full.to_string_lossy().to_string();
    match validate_scroll(&scroll.yaml_metadata) {
        Ok(()) => {
            // On pass: close matching validator thread if exists
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async move {
                let ac = crate::threads::thread_autocapture::ThreadAutocapture::new(&conn);
                let _ = ac.on_validator_pass(&scroll_path).await;
            });
            println!("Validation OK: {}", file);
        }
        Err(e) => {
            // On failure: dedupe_or_open w/ defaults and due_at now+48h
            let title = format!("Validation failed: {}", full.display());
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async move {
                let ac = crate::threads::thread_autocapture::ThreadAutocapture::new(&conn);
                let _ = ac.on_validator_failure(&scroll_path, &title).await;
            });
            return Err(anyhow!(e));
        }
    }
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
                    let conn = ensure_db();
                    let scroll_path = path.to_string_lossy().to_string();
                    match validate_scroll(&scroll.yaml_metadata) {
                        Err(e) => {
                            // failure: open or reuse
                            let title = format!("Validation failed: {}", path.display());
                            let rt = tokio::runtime::Runtime::new()?;
                            rt.block_on(async move {
                                let ac = crate::threads::thread_autocapture::ThreadAutocapture::new(&conn);
                                let _ = ac.on_validator_failure(&scroll_path, &title).await;
                            });
                            eprintln!("{}: invalid – {}", path.display(), e);
                            bad += 1;
                        }
                        Ok(()) => {
                            // pass: close matching thread
                            let rt = tokio::runtime::Runtime::new()?;
                            rt.block_on(async move {
                                let ac = crate::threads::thread_autocapture::ThreadAutocapture::new(&conn);
                                let _ = ac.on_validator_pass(&scroll_path).await;
                            });
                            ok += 1;
                        }
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
    if bad > 0 {
        return Err(anyhow!("Validation errors present"));
    }
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
