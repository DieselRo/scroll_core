use crate::schema::YamlMetadata;
use anyhow::Result;
use pathdiff::diff_paths;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Clone, Debug, Serialize)]
pub struct Finding {
    pub category: String,
    pub primary_path: String,
    pub paths: Vec<String>,
    pub message: String,
    pub code: String,
}

fn hash_str(s: &str) -> String {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    let res = h.finalize();
    let hex = format!("{:x}", res);
    hex[..8].to_string()
}

pub fn doc_scan_contradictions(fix: bool) -> Result<()> {
    let root = super::repo_root();
    let out_dir = root.join("docs").join("reference");
    std::fs::create_dir_all(&out_dir)?;

    let archive_dir = std::env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
    let base = Path::new(&archive_dir);
    if !base.exists() {
        eprintln!("warning: archive dir '{}' does not exist", base.display());
    }

    let mut title_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut name_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut norm_map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for entry in WalkDir::new(base).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext != "md" && ext != "txt" {
            continue;
        }
        let rel = diff_paths(path, &root).unwrap_or_else(|| path.to_path_buf());
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        let norm = if cfg!(windows) {
            rel_str.to_lowercase()
        } else {
            rel_str.clone()
        };
        norm_map.entry(norm).or_default().push(rel_str.clone());
        let fname = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string()
            .to_lowercase();
        name_map.entry(fname).or_default().push(rel_str.clone());

        let content = fs::read_to_string(path).unwrap_or_default();
        let mut title: Option<String> = None;
        if let Some(rest) = content.strip_prefix("---\n") {
            if let Some(end) = rest.find("\n---") {
                let fm = &content[4..4 + end];
                if let Ok(meta) = serde_yaml::from_str::<YamlMetadata>(fm) {
                    if !meta.title.trim().is_empty() {
                        title = Some(meta.title);
                    }
                }
            }
        }
        if title.is_none() {
            for line in content.lines() {
                if let Some(stripped) = line.strip_prefix("# ") {
                    title = Some(stripped.trim().to_string());
                    break;
                }
            }
        }
        let title = title.unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        });
        title_map
            .entry(title.to_lowercase())
            .or_default()
            .push(rel_str);
    }

    let mut findings: Vec<Finding> = Vec::new();
    for (t, paths) in title_map.into_iter() {
        if paths.len() > 1 {
            let code = hash_str(&t);
            let primary = paths[0].clone();
            let msg = format!("Duplicate title '{}': {}", t, paths.join(", "));
            findings.push(Finding {
                category: "duplicate_title".into(),
                primary_path: primary,
                paths,
                message: msg,
                code,
            });
        }
    }
    for (n, paths) in name_map.into_iter() {
        if paths.len() > 1 {
            let code = hash_str(&n);
            let primary = paths[0].clone();
            let msg = format!("File name collision '{}': {}", n, paths.join(", "));
            findings.push(Finding {
                category: "path_collision".into(),
                primary_path: primary,
                paths,
                message: msg,
                code,
            });
        }
    }
    for (n, paths) in norm_map.into_iter() {
        if paths.len() > 1 {
            let code = hash_str(&n);
            let primary = paths[0].clone();
            let msg = format!("Path collision '{}': {}", n, paths.join(", "));
            findings.push(Finding {
                category: "path_collision".into(),
                primary_path: primary,
                paths,
                message: msg,
                code,
            });
        }
    }

    findings.sort_by(|a, b| {
        a.category
            .cmp(&b.category)
            .then(a.primary_path.cmp(&b.primary_path))
    });

    fs::write(
        out_dir.join("doc-contradictions.json"),
        serde_json::to_vec_pretty(&findings)?,
    )?;

    let mut md = String::new();
    md.push_str("# Contradiction Scan Report\n\n");
    md.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().to_rfc3339()
    ));
    if findings.is_empty() {
        md.push_str("No contradictions detected.\n");
    } else {
        let mut cat_map: BTreeMap<&str, Vec<&Finding>> = BTreeMap::new();
        for f in &findings {
            cat_map.entry(&f.category).or_default().push(f);
        }
        for (cat, list) in cat_map {
            md.push_str(&format!("## {}\n", cat));
            for f in list {
                md.push_str(&format!("- {}\n", f.message));
            }
            md.push('\n');
        }
    }
    fs::write(out_dir.join("doc-contradictions.md"), md)?;

    if fix {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            if let Ok(rt) = tokio::runtime::Runtime::new() {
                let findings_clone = findings.clone();
                let fut = async move {
                    if let Ok(conn) = crate::sessions::database::ensure_ready_with_url(&url).await {
                        let ac = crate::threads::thread_autocapture::ThreadAutocapture::new(conn);
                        for f in findings_clone {
                            let _ = ac
                                .on_doc_contradiction(&f.primary_path, &f.code, &f.message)
                                .await;
                        }
                    }
                };
                let _ = rt.block_on(fut);
            }
        }
    }

    println!("Docs: contradiction report written to docs/reference/doc-contradictions.md");
    Ok(())
}
