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
    pub markdown_body: String,
    pub invocation_phrase: String,
    pub sigil: String,
    pub status: ScrollStatus,
    pub emotion_signature: EmotionSignature,
    pub linked_scrolls: Vec<ScrollLink>,
    pub origin: ScrollOrigin,
}

impl Scroll {
    pub fn new(
        title: String,
        scroll_type: ScrollType,
        yaml_metadata: YamlMetadata,
        markdown_body: String,
        invocation_phrase: String,
        sigil: String,
        emotion_signature: EmotionSignature,
        authored_by: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Scroll {
            id: Uuid::new_v4(),
            title,
            scroll_type,
            yaml_metadata,
            markdown_body,
            invocation_phrase,
            sigil,
            status: ScrollStatus::Latent,
            emotion_signature,
            linked_scrolls: Vec::new(),
            origin: ScrollOrigin {
                created: now,
                authored_by,
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
