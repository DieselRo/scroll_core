use crate::schema::{ScrollType, YamlMetadata};
use anyhow::Result;
use pathdiff::diff_paths;
use regex::Regex;
use serde::Serialize;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize)]
struct DocRow {
    path: String,
    href: String,
    category: String,
    size: u64,
    modified: String,
}

fn to_href(from_dir: &Path, target: &Path) -> String {
    let rel = diff_paths(target, from_dir).unwrap_or_else(|| target.to_path_buf());
    rel.to_string_lossy().replace('\\', "/")
}

fn category_of(rel_path: &str) -> String {
    if rel_path.starts_with("scrolls") {
        "scroll".into()
    } else if rel_path.starts_with("docs") {
        "project-doc".into()
    } else if rel_path.starts_with("tests") {
        "test-doc".into()
    } else {
        "root-doc".into()
    }
}

fn repo_root() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn doc_index() -> Result<()> {
    let root = repo_root();
    let out_dir = root.join("docs").join("reference");
    fs::create_dir_all(&out_dir)?;
    let mut rows: Vec<DocRow> = Vec::new();
    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
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
        let rel_path = path
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .trim_start_matches(['\\', '/'])
            .replace('\\', "/");
        let md = entry.metadata().ok();
        let size = md.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = md
            .and_then(|m| m.modified().ok())
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
            .unwrap_or_default();
        let href = to_href(&out_dir, path);
        rows.push(DocRow {
            path: rel_path.clone(),
            href,
            category: category_of(&rel_path),
            size,
            modified,
        });
    }
    // write JSON
    let json_path = out_dir.join("doc-index.json");
    fs::write(&json_path, serde_json::to_vec_pretty(&rows)?)?;
    // write Markdown
    use std::collections::BTreeMap;
    let mut by_dir: BTreeMap<String, Vec<&DocRow>> = BTreeMap::new();
    for row in &rows {
        let dir = Path::new(&row.path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "".into());
        by_dir.entry(dir).or_default().push(row);
    }
    let mut md_buf = String::new();
    md_buf.push_str("# Document Index\n\n");
    md_buf.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().to_rfc3339()
    ));
    for (dir, files) in by_dir {
        md_buf.push_str(&format!("## {}\n", dir));
        for f in files {
            let name = Path::new(&f.path).file_name().unwrap().to_string_lossy();
            md_buf.push_str(&format!("- [{}]({})\n", name, f.href));
        }
        md_buf.push('\n');
    }
    fs::write(out_dir.join("doc-index.md"), md_buf)?;
    Ok(())
}

pub fn doc_recent() -> Result<()> {
    let root = repo_root();
    let out_dir = root.join("docs").join("reference");
    fs::create_dir_all(&out_dir)?;
    let mut rows: Vec<(std::time::SystemTime, DocRow)> = Vec::new();
    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
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
        let rel_path = path
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .trim_start_matches(['\\', '/'])
            .replace('\\', "/");
        let md = entry.metadata().ok();
        let size = md.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = md
            .as_ref()
            .and_then(|m| m.modified().ok())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let modified_str = chrono::DateTime::<chrono::Utc>::from(modified).to_rfc3339();
        let href = to_href(&out_dir, path);
        rows.push((
            modified,
            DocRow {
                path: rel_path.clone(),
                href,
                category: category_of(&rel_path),
                size,
                modified: modified_str,
            },
        ));
    }
    rows.sort_by_key(|(t, _)| std::cmp::Reverse(*t));
    let mut md_buf = String::new();
    md_buf.push_str("# Recent Documents\n\n");
    md_buf.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().to_rfc3339()
    ));
    for (_, row) in rows.iter().take(200) {
        md_buf.push_str(&format!(
            "- [{}]({})  — {}\n",
            row.path, row.href, row.modified
        ));
    }
    fs::write(out_dir.join("doc-recent.md"), md_buf)?;
    Ok(())
}

pub fn doc_classify() -> Result<()> {
    let root = repo_root();
    let out_dir = root.join("docs").join("reference");
    fs::create_dir_all(&out_dir)?;
    // header detection
    let key_re = Regex::new(r"(?m)^\s*(scroll_type|invocation_phrase|sigil|tags|emotion_signature|quorum_required|archetype|inscribed|author|status)\s*:").unwrap();
    #[derive(Serialize)]
    struct ClassRow {
        path: String,
        href: String,
        category: String,
        has_yaml_front_matter: bool,
        scroll_header_key_count: usize,
        is_scroll_by_header: bool,
        is_scroll_by_path: bool,
        topic: String,
    }
    let mut rows: Vec<ClassRow> = Vec::new();
    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
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
        let rel_path = path
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .trim_start_matches(['\\', '/'])
            .replace('\\', "/");
        let href = to_href(&out_dir, path);
        let category = category_of(&rel_path);
        let content = fs::read_to_string(path).unwrap_or_default();
        let has_yaml = content.starts_with("---\n");
        let key_count = if has_yaml {
            // parse until next ---
            if let Some(end) = content[4..].find("\n---") {
                let fm = &content[4..4 + end];
                key_re.captures_iter(fm).count()
            } else {
                0
            }
        } else {
            0
        };
        let is_header = has_yaml && key_count >= 2;
        let is_path = rel_path.starts_with("scrolls");
        // naive topic from filename
        let lower = rel_path.to_lowercase();
        let topic = if lower.contains("protocol") {
            "protocol"
        } else if lower.contains("ritual") {
            "ritual"
        } else if lower.contains("invocation") {
            "invocation"
        } else if lower.contains("loom") || lower.contains("trigger") {
            "loom"
        } else {
            ""
        };
        rows.push(ClassRow {
            path: rel_path,
            href,
            category,
            has_yaml_front_matter: has_yaml,
            scroll_header_key_count: key_count,
            is_scroll_by_header: is_header,
            is_scroll_by_path: is_path,
            topic: topic.into(),
        });
    }
    fs::write(
        out_dir.join("doc-manifest.json"),
        serde_json::to_vec_pretty(&rows)?,
    )?;
    // ambiguous report
    let mut md_buf = String::new();
    md_buf.push_str("# Ambiguous or Cross-Boundary Documents\n\n");
    md_buf.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().to_rfc3339()
    ));
    let non_scroll_header: Vec<&ClassRow> = rows
        .iter()
        .filter(|r| r.category != "scroll" && r.is_scroll_by_header)
        .collect();
    let scroll_no_header: Vec<&ClassRow> = rows
        .iter()
        .filter(|r| r.category == "scroll" && !r.is_scroll_by_header)
        .collect();
    md_buf.push_str(&format!(
        "## Not under scrolls/, but has scroll header ({})\n",
        non_scroll_header.len()
    ));
    for r in &non_scroll_header {
        md_buf.push_str(&format!(
            "- [{}]({}) — {} (keys: {})\n",
            r.path, r.href, r.category, r.scroll_header_key_count
        ));
    }
    md_buf.push('\n');
    md_buf.push_str(&format!(
        "## Under scrolls/, but missing scroll header ({})\n",
        scroll_no_header.len()
    ));
    for r in &scroll_no_header {
        md_buf.push_str(&format!("- [{}]({})\n", r.path, r.href));
    }
    fs::write(out_dir.join("doc-ambiguous.md"), md_buf)?;
    Ok(())
}

fn infer_scroll_type_from_name(lower: &str) -> &'static str {
    if lower.contains("scrollbook") {
        "Scrollbook"
    } else if lower.contains("protocol") {
        "Protocol"
    } else if lower.contains("ritual") {
        "Ritual"
    } else if lower.contains("system") {
        "System"
    } else if lower.contains("myth") {
        "Myth"
    } else if lower.contains("agent") {
        "AgentCatalog"
    } else {
        "Canon"
    }
}

fn suggest_header_for(path: &Path, _body: &str) -> String {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled");
    let title = stem.replace(['_', '-'], " ");
    let lower_name = stem.to_lowercase();
    let scroll_type = infer_scroll_type_from_name(&lower_name);
    // minimal valid header for validator
    let yaml = format!(
        "---\ntitle: \"{}\"\nscroll_type: {}\nemotion_signature:\n  tone: neutral\n  emphasis: 0.0\n  resonance: balanced\n  intensity: 0.0\ntags:\n  - imported\n---\n",
        title, scroll_type
    );
    // Return header only; caller will concatenate with body
    yaml
}

pub fn doc_fix_headers() -> Result<()> {
    let root = repo_root();
    let scrolls_dir = root.join("scrolls");
    if !scrolls_dir.exists() {
        return Ok(());
    }
    for entry in WalkDir::new(&scrolls_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
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
        let content = fs::read_to_string(path).unwrap_or_default();
        let has_yaml = content.starts_with("---\n");
        let mut needs_fix = false;
        let mut new_content = String::new();
        if has_yaml {
            // replace existing front matter if missing required keys
            if let Some(end_idx) = content[4..].find("\n---") {
                let header = &content[4..4 + end_idx];
                // check minimal keys
                let has_title = header.contains("\ntitle:") || header.starts_with("title:");
                let has_type =
                    header.contains("\nscroll_type:") || header.starts_with("scroll_type:");
                let has_tags = header.contains("\ntags:") || header.starts_with("tags:");
                let has_emotion = header.contains("\nemotion_signature:")
                    || header.starts_with("emotion_signature:");
                if !(has_title && has_type && has_tags && has_emotion) {
                    let body = &content[(4 + end_idx + 4)..]; // skip leading '---\n' (4 incl. newline) + header + '\n---'
                    let header_yaml = suggest_header_for(path, body);
                    new_content.push_str(&header_yaml);
                    new_content.push_str(body);
                    needs_fix = true;
                }
            } else {
                // malformed (no closing ---)
                let body = content; // treat entire content as body
                let header_yaml = suggest_header_for(path, &body);
                new_content.push_str(&header_yaml);
                new_content.push_str(&body);
                needs_fix = true;
            }
        } else {
            // no header present
            let header_yaml = suggest_header_for(path, &content);
            new_content.push_str(&header_yaml);
            new_content.push_str(&content);
            needs_fix = true;
        }
        if needs_fix {
            fs::write(path, new_content)?;
        }
    }
    Ok(())
}

pub fn doc_normalize_headers() -> Result<()> {
    let root = repo_root();
    let scrolls_dir = root.join("scrolls");
    let out_dir = root.join("docs").join("reference");
    fs::create_dir_all(&out_dir)?;

    let mut changed: Vec<(String, usize)> = Vec::new();
    for entry in WalkDir::new(&scrolls_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
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
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        let content = fs::read_to_string(path).unwrap_or_default();
        if !content.starts_with("---\n") {
            continue;
        }
        if let Some(end_idx) = content[4..].find("\n---") {
            let header = &content[4..4 + end_idx];
            let body = &content[(4 + end_idx + 4)..];
            // Deserialize leniently to canonical YamlMetadata
            match serde_yaml::from_str::<YamlMetadata>(header) {
                Ok(meta) => {
                    let yaml = serde_yaml::to_string(&meta).unwrap_or_default();
                    let normalized = format!("---\n{}---\n{}", yaml, body);
                    if normalized != content {
                        fs::write(path, &normalized)?;
                        // Count number of changes as lines in header
                        let lines = header.lines().count();
                        changed.push((rel, lines));
                    }
                }
                Err(_) => { /* skip malformed; other fixers handle this */ }
            }
        }
    }
    // Report
    let mut report = String::new();
    report.push_str("# Header Normalization Report\n\n");
    report.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().to_rfc3339()
    ));
    report.push_str(&format!("Files normalized: {}\n\n", changed.len()));
    for (file, _cnt) in &changed {
        report.push_str(&format!("- {}\n", file));
    }
    fs::write(out_dir.join("doc-normalize-report.md"), report)?;

    // Append to CHANGELOG
    let changelog_path = out_dir.join("CHANGELOG.md");
    let mut entry = String::new();
    entry.push_str(&format!(
        "\n- {}: normalized headers for {} scroll(s)\n",
        chrono::Utc::now().to_rfc3339(),
        changed.len()
    ));
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&changelog_path)?
        .write_all(entry.as_bytes());
    Ok(())
}

pub fn doc_generate_master_plan() -> Result<()> {
    let root = repo_root();
    let out_dir = root.join("docs").join("alignment");
    fs::create_dir_all(&out_dir)?;

    #[derive(Serialize)]
    struct Entry {
        title: String,
        scroll_type: String,
        rel: String,
        modified: String,
    }
    let mut entries: Vec<Entry> = Vec::new();
    for entry in WalkDir::new(root.join("scrolls"))
        .into_iter()
        .filter_map(Result::ok)
    {
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
        // Skip deactivated
        let path_str = path.to_string_lossy();
        if path_str.contains("Deactivated scrolls") {
            continue;
        }
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let content = fs::read_to_string(path).unwrap_or_default();
        let mut title = String::from("Untitled Scroll");
        let mut stype = String::from("Echo");
        if let Some(stripped) = content.strip_prefix("---\n") {
            if let Some(end) = stripped.find("\n---") {
                let fm = &content[4..4 + end];
                if let Ok(meta) = serde_yaml::from_str::<YamlMetadata>(fm) {
                    title = meta.title;
                    stype = meta.scroll_type.to_string();
                }
            }
        }
        // Fallback title from first Markdown heading
        if title == "Untitled Scroll" {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("# ") {
                    title = trimmed.trim_start_matches("# ").trim().to_string();
                    break;
                }
            }
        }
        // Fallback to file stem prettified
        if title == "Untitled Scroll" {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                title = stem.replace(['_', '-'], " ");
            }
        }
        let modified = entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
            .unwrap_or_default();
        entries.push(Entry {
            title,
            scroll_type: stype,
            rel,
            modified,
        });
    }
    entries.sort_by(|a, b| b.modified.cmp(&a.modified));

    let mut md = String::new();
    md.push_str("---\n");
    md.push_str("title: Master Plan\nstatus: active\naudience: user, dev\n---\n\n");
    md.push_str("# Master Plan\n\n");
    md.push_str(
        "This plan synthesizes the current archive, immediate priorities, and runtime roadmap.\n\n",
    );
    md.push_str("## Core Purpose\n");
    md.push_str("Scroll Core lets named AI Constructs read/write/manage scrolls (YAML+Markdown) as a local-first AI runtime with persistent, structured knowledge.\n\n");
    md.push_str("## Immediate Priorities\n- Solidify CLI and docs tooling (index/recent/classify/normalize)\n- Ensure all active scrolls load with tolerant parsing\n- LumenMind (frontend) skeleton with construct selector\n\n");
    md.push_str("## Recent Key Scrolls\n");
    for e in entries.iter().take(30) {
        md.push_str(&format!("- [{}]({}) — {}\n", e.title, e.rel, e.scroll_type));
    }
    md.push_str("\n## By Type\n");
    for t in [
        ScrollType::Scrollbook,
        ScrollType::Canon,
        ScrollType::Protocol,
        ScrollType::System,
        ScrollType::Myth,
        ScrollType::Ritual,
        ScrollType::Echo,
    ] {
        let tlabel = t.to_string();
        let list: Vec<&Entry> = entries
            .iter()
            .filter(|e| e.scroll_type == tlabel)
            .take(10)
            .collect();
        if !list.is_empty() {
            md.push_str(&format!("### {}\n", tlabel));
            for e in list {
                md.push_str(&format!("- [{}]({})\n", e.title, e.rel));
            }
            md.push('\n');
        }
    }
    fs::write(out_dir.join("MASTER_PLAN.md"), md)?;
    Ok(())
}

// ───────────────────────────────────────────────────────────────────────────────
// Contradiction Scanner (skeleton)
// ───────────────────────────────────────────────────────────────────────────────
#[derive(Default, Debug, serde::Serialize)]
struct ContradictionReport {
    duplicate_titles: Vec<String>,
    duplicate_paths: Vec<String>,
    // Future: conflicting types/tags across same topic, etc.
}

pub fn doc_scan_contradictions(fix: bool) -> Result<()> {
    let root = repo_root();
    let out_dir = root.join("docs").join("reference");
    std::fs::create_dir_all(&out_dir)?;

    // Determine archive dir from env for testability
    let archive_dir = std::env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
    let base = Path::new(&archive_dir);
    if !base.exists() {
        eprintln!("warning: archive dir '{}' does not exist", base.display());
    }

    // Simple pass: track titles and paths
    use std::collections::{BTreeMap, BTreeSet};
    let mut titles: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut paths_seen: BTreeSet<String> = BTreeSet::new();
    let mut duplicate_paths: BTreeSet<String> = BTreeSet::new();

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
        if !paths_seen.insert(rel_str.clone()) {
            duplicate_paths.insert(rel_str.clone());
        }
        let content = std::fs::read_to_string(path).unwrap_or_default();
        let mut title = None::<String>;
        if let Some(rest) = content.strip_prefix("---\n") {
            if let Some(end) = rest.find("\n---") {
                let fm = &content[4..4 + end];
                if let Ok(meta) = serde_yaml::from_str::<YamlMetadata>(fm) {
                    title = Some(meta.title);
                }
            }
        }
        let t = title.unwrap_or_else(|| path.file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string());
        titles.entry(t).or_default().insert(rel_str);
    }

    let mut duplicate_titles = Vec::new();
    for (t, set) in titles {
        if set.len() > 1 {
            duplicate_titles.push(format!("{} → {}", t, set.into_iter().collect::<Vec<_>>().join(", ")));
        }
    }
    duplicate_titles.sort();

    let mut duplicate_paths_vec = duplicate_paths.into_iter().collect::<Vec<_>>();
    duplicate_paths_vec.sort();

    let report = ContradictionReport {
        duplicate_titles: duplicate_titles.clone(),
        duplicate_paths: duplicate_paths_vec.clone(),
    };
    // Write machine-readable JSON
    std::fs::write(out_dir.join("doc-contradictions.json"), serde_json::to_vec_pretty(&report)?)?;
    // Write human-readable Markdown
    let mut md = String::new();
    md.push_str("# Contradiction Scan Report\n\n");
    md.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().to_rfc3339()));
    if duplicate_titles.is_empty() && duplicate_paths_vec.is_empty() {
        md.push_str("No contradictions detected by the skeleton scanner.\n");
    } else {
        if !duplicate_titles.is_empty() {
            md.push_str("## Duplicate Titles\n");
            for row in &duplicate_titles {
                md.push_str(&format!("- {}\n", row));
            }
            md.push('\n');
        }
        if !duplicate_paths_vec.is_empty() {
            md.push_str("## Duplicate Paths\n");
            for row in &duplicate_paths_vec {
                md.push_str(&format!("- {}\n", row));
            }
            md.push('\n');
        }
    }
    std::fs::write(out_dir.join("doc-contradictions.md"), md)?;

    if fix {
        // Skeleton fix strategy: open a placeholder thread per duplicate title group
        // if DB is configured. Failures are non-fatal.
        if let Ok(url) = std::env::var("DATABASE_URL") {
            if let Ok(rt) = tokio::runtime::Runtime::new() {
                let fut = async move {
                    if let Ok(_conn) = crate::sessions::database::ensure_ready_with_url(&url).await
                    {
                        let ac = crate::threads::thread_autocapture::ThreadAutocapture::new(
                            &crate::sessions::database::get_db_connection(),
                        );
                        for row in duplicate_titles {
                            let title = format!("Contradiction: duplicate title — {}", row);
                            let _ = ac.on_validator_failure("<doc-scan>", &title).await;
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
