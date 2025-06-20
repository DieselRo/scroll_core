// scroll.rs

use chrono::{DateTime, Utc};
use log::info;
use std::fmt;
use uuid::Uuid;

use crate::schema::{EmotionSignature, ScrollStatus, ScrollType, YamlMetadata};

#[derive(Clone, PartialEq, Debug)]
pub struct ScrollOrigin {
    pub created: DateTime<Utc>,
    pub authored_by: Option<String>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ScrollLinkType {
    Ancestor,
    Reflection,
    Derivative,
    Binding,
    Echo,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ScrollLink {
    pub target: Uuid,
    pub link_type: ScrollLinkType,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Scroll {
    pub id: Uuid,
    pub title: String,
    pub scroll_type: ScrollType,
    pub yaml_metadata: YamlMetadata,
    pub tags: Vec<String>,
    pub archetype: Option<String>,
    pub quorum_required: bool,
    pub markdown_body: String,
    pub invocation_phrase: String,
    pub sigil: String,
    pub status: ScrollStatus,
    pub emotion_signature: EmotionSignature,
    pub linked_scrolls: Vec<ScrollLink>,
    pub origin: ScrollOrigin,
}

pub struct ScrollBuilder {
    pub title: String,
    pub scroll_type: ScrollType,
    pub yaml_metadata: YamlMetadata,
    pub tags: Vec<String>,
    pub archetype: Option<String>,
    pub quorum_required: bool,
    pub markdown_body: String,
    pub invocation_phrase: String,
    pub sigil: String,
    pub emotion_signature: EmotionSignature,
    pub authored_by: Option<String>,
}

impl ScrollBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            title: title.clone(),
            scroll_type: ScrollType::Canon,
            yaml_metadata: YamlMetadata {
                title,
                scroll_type: ScrollType::Canon,
                emotion_signature: EmotionSignature::neutral(),
                tags: vec![],
                archetype: None,
                quorum_required: false,
                last_modified: None,
                file_path: None,
            },
            tags: vec![],
            archetype: None,
            quorum_required: false,
            markdown_body: String::new(),
            invocation_phrase: String::new(),
            sigil: String::new(),
            emotion_signature: EmotionSignature::neutral(),
            authored_by: None,
        }
    }

    pub fn tags(mut self, tags: &[&str]) -> Self {
        let collected: Vec<String> = tags.iter().map(|t| t.to_string()).collect();
        self.tags = collected.clone();
        self.yaml_metadata.tags = collected;
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.markdown_body = body.into();
        self
    }

    pub fn invocation_phrase(mut self, phrase: impl Into<String>) -> Self {
        self.invocation_phrase = phrase.into();
        self
    }

    pub fn sigil(mut self, sigil: impl Into<String>) -> Self {
        self.sigil = sigil.into();
        self
    }

    pub fn last_modified(mut self, dt: DateTime<Utc>) -> Self {
        self.yaml_metadata.last_modified = Some(dt);
        self
    }

    pub fn build(self) -> Scroll {
        let mut scroll = Scroll::new(self);
        scroll.status = ScrollStatus::Draft;
        scroll
    }
}

impl Scroll {
    pub fn builder(title: impl Into<String>) -> ScrollBuilder {
        ScrollBuilder::new(title)
    }

    pub fn new(params: ScrollBuilder) -> Self {
        let now = Utc::now();
        Scroll {
            id: Uuid::new_v4(),
            title: params.title,
            scroll_type: params.scroll_type,
            yaml_metadata: params.yaml_metadata,
            tags: params.tags,
            archetype: params.archetype,
            quorum_required: params.quorum_required,
            markdown_body: params.markdown_body,
            invocation_phrase: params.invocation_phrase,
            sigil: params.sigil,
            status: ScrollStatus::Latent,
            emotion_signature: params.emotion_signature,
            linked_scrolls: Vec::new(),
            origin: ScrollOrigin {
                created: now,
                authored_by: params.authored_by,
                last_modified: now,
            },
        }
    }

    pub fn seal(&mut self) {
        self.status = ScrollStatus::Sealed;
        self.origin.last_modified = Utc::now();
        info!(
            "Scroll '{}' sealed at {:?}",
            self.title, self.origin.last_modified
        );
    }

    pub fn awaken(&mut self) {
        self.status = ScrollStatus::Emergent;
        self.origin.last_modified = Utc::now();
        info!(
            "Scroll '{}' awakened at {:?}",
            self.title, self.origin.last_modified
        );
    }

    pub fn link_to(&mut self, other: &Scroll, link_type: ScrollLinkType) {
        self.linked_scrolls.push(ScrollLink {
            target: other.id,
            link_type: link_type.clone(),
        });
        self.origin.last_modified = Utc::now();
        info!(
            "Scroll '{}' linked to '{}' as {:?}.",
            self.title, other.title, link_type
        );
    }

    pub fn is_linked_to(&self, other_id: &Uuid) -> bool {
        self.linked_scrolls
            .iter()
            .any(|link| &link.target == other_id)
    }

    pub fn is_symbolically_valid(&self) -> bool {
        !self.sigil.is_empty()
            && matches!(self.scroll_type, ScrollType::Canon | ScrollType::Scrollbook)
            && !self.invocation_phrase.is_empty()
            && !self.emotion_signature.is_empty()
    }

    pub fn echo_emotion(&self) -> String {
        format!("{} resonates with {}", self.title, self.emotion_signature)
    }

    pub fn echo_emotion_symbolic(&self) -> String {
        format!(
            "📜 {} ∴ 🔮 {} ✦ {}",
            self.title, self.emotion_signature.tone, self.emotion_signature.resonance
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.is_symbolically_valid() {
            return Err("Scroll is not symbolically valid.".into());
        }
        if self.title.trim().is_empty() {
            return Err("Scroll title is missing.".into());
        }
        Ok(())
    }

    pub fn to_poetic(&self) -> String {
        format!(
            "[{}] • '{}' stirs beneath {:?} — a voice wrapped in {} and whispered by '{}'.",
            self.scroll_type,
            self.title,
            self.status,
            self.emotion_signature,
            self.origin
                .authored_by
                .as_deref()
                .unwrap_or("an Unknown Voice")
        )
    }
}

impl fmt::Display for Scroll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Scroll '{}': [{}] — Status: {:?}\nInvocation: '{}'\nEmotion: {}\nCreated: {:?} by {}\nLast Modified: {:?}",
            self.title,
            self.scroll_type,
            self.status,
            self.invocation_phrase,
            self.emotion_signature,
            self.origin.created,
            self.origin.authored_by.as_deref().unwrap_or("Unknown"),
            self.origin.last_modified
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScrollDraft {
    pub title: String,
    pub content: String,
}
