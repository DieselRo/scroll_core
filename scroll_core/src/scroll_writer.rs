// scroll_writer.rs – Hand of the Archive
//===========================================
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::{File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use serde_yaml;
use chrono::Utc;


use crate::scroll::{ScrollOrigin, Scroll};
use crate::schema::{ScrollStatus, ScrollType, EmotionSignature, YamlMetadata};
use crate::parser::parse_scroll_from_file;
use crate::validator::validate_scroll;
use crate::artifact::WritableArtifact;



/// Patch structure for updating existing scroll fields.
pub struct ScrollPatch {
    pub title: Option<String>,
    pub markdown_body: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sigil: Option<String>,
}

impl WritableArtifact for Scroll {
    fn to_string_representation(&self) -> String {
        let yaml = serde_yaml::to_string(&self.yaml_metadata)
            .unwrap_or_else(|_| "error: could not serialize metadata".to_string());

        format!(
            "---\n{}---\n\n{}",
            yaml.trim(),
            self.markdown_body.trim()
        )
    }

    fn file_extension(&self) -> &'static str {
        "md"
    }
}

pub struct ArtifactWriter;

impl ArtifactWriter {
    /// Writes any artifact implementing WritableArtifact to disk.
    pub fn write_artifact<A: WritableArtifact>(artifact: &A, path: &Path) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(artifact.to_string_representation().as_bytes())
            .map_err(|e| e.to_string())
    }
}

pub struct ScrollWriter;

impl ScrollWriter {
    /// Writes a scroll to disk as a markdown file.
    pub fn write_scroll(scroll: &Scroll, path: &Path) -> Result<(), String> {
        validate_scroll(&scroll.yaml_metadata).map_err(|e| format!("Validation failed: {}", e))?;
        ArtifactWriter::write_artifact(scroll, path)
    }

    /// Applies patch and updates an existing scroll.
    pub fn update_scroll(_id: Uuid, updates: ScrollPatch, path: &Path) -> Result<(), String> {
        let mut scroll = parse_scroll_from_file(path)?;

        if let Some(title) = updates.title {
               scroll.title = title.clone();
               scroll.yaml_metadata.title = title;
        }

        if let Some(body) = updates.markdown_body {
               scroll.markdown_body = body;
        }

        if let Some(tags) = updates.tags {
               scroll.yaml_metadata.tags = tags;
        
        }

        if let Some(sigil) = updates.sigil {
               scroll.sigil = sigil;
        }

        let now = chrono::Utc::now();
        scroll.origin.last_modified = now;
        scroll.yaml_metadata.last_modified = Some(now);
        Self::write_scroll(&scroll, path)
    }

    /// Marks a scroll as sealed.
    pub fn seal_scroll(scroll: &mut Scroll) -> Result<(), String> {
        scroll.status = ScrollStatus::Sealed;
        scroll.origin.last_modified = chrono::Utc::now();
        Ok(())
    }

    /// Creates a draft scroll from symbolic input.
    pub fn generate_draft(
    title: String,
    scroll_type: ScrollType,
    emotion: EmotionSignature,
    tags: Vec<String>,
) -> Scroll {
    let now = Utc::now();
    let title_clone = title.clone();
    let scroll_type_clone = scroll_type.clone();

    Scroll {
        id: Uuid::new_v4(),
        title,
        scroll_type,
        yaml_metadata: YamlMetadata {
            title: title_clone,
            scroll_type: scroll_type_clone,
            emotion_signature: emotion.clone(),
            tags,
            last_modified: Some(now),
  
        },

        markdown_body: String::new(),
        invocation_phrase: String::new(),
        sigil: String::new(),
        status: ScrollStatus::Draft,
        emotion_signature: emotion,
        linked_scrolls: vec![],
        origin: ScrollOrigin {
           created: now,
           authored_by: None,
           last_modified: now,

        },
    }
  }
        }
