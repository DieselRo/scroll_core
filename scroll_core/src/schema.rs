// ===============================
// src/schema.rs
// ===============================
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScrollType {
    Canon,
    Protocol,
    System,
    Scrollbook,
    AgentCatalog,
    Myth,
    Ritual,

    #[default]
    Echo,
}

impl fmt::Display for ScrollType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            ScrollType::Canon => "Canon",
            ScrollType::Protocol => "Protocol",
            ScrollType::Myth => "Myth",
            ScrollType::System => "System",
            ScrollType::Scrollbook => "Scrollbook",
            ScrollType::AgentCatalog => "AgentCatalog",
            ScrollType::Echo => "Echo",
            ScrollType::Ritual => "Ritual",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScrollStatus {
    Emergent,
    Draft,
    Active,
    Sealed,
    Archived,
    Latent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmotionSignature {
    pub tone: String,
    pub emphasis: f32,
    pub resonance: String,
    pub intensity: Option<f32>,
}

impl EmotionSignature {
    pub fn neutral() -> Self {
        Self {
            tone: "neutral".into(),
            emphasis: 0.0,
            resonance: "balanced".into(),
            intensity: Some(0.0),
        }
    }

    pub fn reflective() -> Self {
        Self {
            tone: "calm".into(),
            emphasis: 0.4,
            resonance: "deep".into(),
            intensity: Some(0.2),
        }
    }

    pub fn curious() -> Self {
        Self {
            tone: "inquiring".into(),
            emphasis: 0.6,
            resonance: "seeking".into(),
            intensity: Some(0.5),
        }
    }

    pub fn urgent() -> Self {
        Self {
            tone: "alert".into(),
            emphasis: 0.8,
            resonance: "pressured".into(),
            intensity: Some(0.9),
        }
    }

    pub fn mythic() -> Self {
        Self {
            tone: "eternal".into(),
            emphasis: 0.9,
            resonance: "resonant".into(),
            intensity: Some(0.6),
        }
    }

    pub fn solemn() -> Self {
        Self {
            tone: "somber".into(),
            emphasis: 0.5,
            resonance: "grave".into(),
            intensity: Some(0.4),
        }
    }

    pub fn reverent() -> Self {
        Self {
            tone: "humble".into(),
            emphasis: 0.3,
            resonance: "sacred".into(),
            intensity: Some(0.2),
        }
    }

    pub fn inspired() -> Self {
        Self {
            tone: "bright".into(),
            emphasis: 0.7,
            resonance: "soaring".into(),
            intensity: Some(0.7),
        }
    }

    pub fn frenzied() -> Self {
        Self {
            tone: "chaotic".into(),
            emphasis: 1.0,
            resonance: "unstable".into(),
            intensity: Some(1.0),
        }
    }

    pub fn ancient() -> Self {
        Self {
            tone: "silent".into(),
            emphasis: 0.2,
            resonance: "echoic".into(),
            intensity: Some(0.1),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tone.is_empty() && self.resonance.is_empty() && self.intensity.unwrap_or(0.0) == 0.0
    }
}

impl Default for EmotionSignature {
    fn default() -> Self {
        Self::neutral()
    }
}

impl fmt::Display for EmotionSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} // {} ({:.2})",
            self.tone,
            self.resonance,
            self.intensity.unwrap_or(0.0)
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YamlMetadata {
    pub title: String,
    pub scroll_type: ScrollType,
    pub emotion_signature: EmotionSignature,
    pub tags: Vec<String>,
    #[serde(default)]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub file_path: Option<String>,
}
