// ===============================
// src/parser.rs
// ===============================

use std::fs;
use std::path::Path;

use uuid::Uuid;

use crate::schema::{EmotionSignature, ScrollStatus, ScrollType, YamlMetadata};

use crate::scroll::{Scroll, ScrollOrigin};
use crate::validator::validate_scroll;
use anyhow::{anyhow, Result};

pub fn parse_scroll_from_file<P: AsRef<Path>>(path: P) -> Result<Scroll> {
    let contents = fs::read_to_string(&path).map_err(|e| anyhow!(e))?;
    parse_scroll(&contents)
}

pub fn parse_scroll(input: &str) -> Result<Scroll> {
    let (yaml_str_opt, markdown_body) = extract_yaml_and_markdown_lenient(input)?;
    let yaml_metadata: YamlMetadata = if let Some(yaml_str) = yaml_str_opt {
        serde_yaml::from_str(yaml_str).map_err(|e| anyhow!(e))?
    } else {
        // Minimal fallback metadata if header missing; allows loading but marks as Draft Echo
        YamlMetadata {
            title: "Untitled Scroll".into(),
            scroll_type: ScrollType::Echo,
            emotion_signature: EmotionSignature::neutral(),
            tags: vec!["imported".into()],
            archetype: None,
            quorum_required: false,
            last_modified: None,
            file_path: None,
        }
    };
    validate_scroll(&yaml_metadata).map_err(|e| anyhow!(e))?;

    let emotion_signature = yaml_metadata.emotion_signature.clone();
    let scroll_type = yaml_metadata.scroll_type.clone();
    let title = yaml_metadata.title.clone();
    let now = chrono::Utc::now();

    Ok(Scroll {
        id: Uuid::new_v4(),
        title,
        scroll_type,
        yaml_metadata,
        tags: Vec::new(),
        archetype: None,
        quorum_required: false,
        markdown_body: markdown_body.to_string(),
        invocation_phrase: String::from("Let form meet function in code and myth."),
        sigil: String::from("🔧"),
        status: ScrollStatus::Draft,
        emotion_signature,
        linked_scrolls: vec![],
        origin: ScrollOrigin {
            created: now,
            last_modified: now,
            authored_by: None,
        },
    })
}

fn extract_yaml_and_markdown_lenient(input: &str) -> Result<(Option<&str>, &str)> {
    let trimmed = input.trim_start();
    if trimmed.starts_with("---\n") {
        let parts: Vec<&str> = trimmed.splitn(3, "---").collect();
        if parts.len() >= 3 {
            return Ok((Some(parts[1]), parts[2]));
        } else {
            // Malformed header; treat all as body
            return Ok((None, input));
        }
    }
    Ok((None, input))
}
