// ===============================
// src/schema.rs
// ===============================

use serde::{Serialize, Deserialize}; // make sure this is imported
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// ✅ Added PartialEq for ScrollStatus comparison
pub enum ScrollType {
    Canon,
    Protocol,
    System,
    Scrollbook,
    AgentCatalog,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// ✅ Added PartialEq for ScrollStatus comparison
pub enum ScrollStatus {
    Draft,
    Active,
    Sealed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmotionSignature {
    pub tone: String,
    pub emphasis: f32,
    pub resonance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlMetadata {
    pub title: String,
    pub scroll_type: ScrollType,
    pub emotion_signature: EmotionSignature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollOrigin {
    pub created: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct Scroll {
    pub id: Uuid,
    pub title: String,
    pub scroll_type: ScrollType,
    pub yaml_metadata: YamlMetadata,
    pub markdown_body: String,
    pub invocation_phrase: String,
    pub sigil: String,
    pub status: ScrollStatus,
    pub emotion_signature: EmotionSignature,
    pub linked_scrolls: Vec<Uuid>,
    pub origin: ScrollOrigin,
}