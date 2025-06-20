// ===============================
// src/parser.rs
// ===============================

use std::fs;
use std::path::Path;

use uuid::Uuid;

use crate::schema::{ScrollStatus, YamlMetadata};

use crate::scroll::{Scroll, ScrollOrigin};
use crate::validator::validate_scroll;
use anyhow::{anyhow, Result};

pub fn parse_scroll_from_file<P: AsRef<Path>>(path: P) -> Result<Scroll> {
    let contents = fs::read_to_string(&path).map_err(|e| anyhow!(e))?;
    parse_scroll(&contents)
}

pub fn parse_scroll(input: &str) -> Result<Scroll> {
    let (yaml_str, markdown_body) = extract_yaml_and_markdown(input)?;
    let yaml_metadata: YamlMetadata = serde_yaml::from_str(yaml_str).map_err(|e| anyhow!(e))?;
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

fn extract_yaml_and_markdown(input: &str) -> Result<(&str, &str)> {
    let parts: Vec<&str> = input.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(anyhow!("Invalid format: missing YAML delimiters"));
    }
    Ok((parts[1], parts[2]))
}
