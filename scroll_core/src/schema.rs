// ===============================
// src/schema.rs
// ===============================

use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

impl<'de> Deserialize<'de> for ScrollType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let norm = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        let mapped = match norm.as_str() {
            "canon" => ScrollType::Canon,
            "protocol" => ScrollType::Protocol,
            "system" => ScrollType::System,
            "scrollbook" => ScrollType::Scrollbook,
            "agentcatalog" | "agent" | "catalog" => ScrollType::AgentCatalog,
            "myth" => ScrollType::Myth,
            "ritual" => ScrollType::Ritual,
            "echo" => ScrollType::Echo,
            _ => ScrollType::Echo, // lenient fallback
        };
        Ok(mapped)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ScrollStatus {
    Emergent,
    Draft,
    Active,
    MythicValidated,
    Sealed,
    Archived,
    Latent,
    Deprecated,
}

impl<'de> Deserialize<'de> for ScrollStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let norm = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        let mapped = match norm.as_str() {
            "emergent" => ScrollStatus::Emergent,
            "draft" => ScrollStatus::Draft,
            "active" => ScrollStatus::Active,
            "mythicvalidated" | "validated" => ScrollStatus::MythicValidated,
            "sealed" => ScrollStatus::Sealed,
            "archived" => ScrollStatus::Archived,
            "latent" => ScrollStatus::Latent,
            "deprecated" => ScrollStatus::Deprecated,
            _ => ScrollStatus::Draft,
        };
        Ok(mapped)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
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

#[derive(Deserialize)]
#[serde(untagged)]
enum EmotionSigRepr {
    Str(String),
    Obj {
        tone: String,
        #[serde(default = "default_emphasis")] 
        emphasis: f32,
        resonance: String,
        #[serde(default)]
        intensity: Option<f32>,
    },
}

fn default_emphasis() -> f32 { 0.0 }

fn preset_from_str(s: &str) -> EmotionSignature {
    let key = s.trim().to_lowercase();
    match key.as_str() {
        "neutral" => EmotionSignature::neutral(),
        "reflective" => EmotionSignature::reflective(),
        "curious" => EmotionSignature::curious(),
        "urgent" => EmotionSignature::urgent(),
        "mythic" => EmotionSignature::mythic(),
        "solemn" => EmotionSignature::solemn(),
        "reverent" | "reverence" => EmotionSignature::reverent(),
        "inspired" => EmotionSignature::inspired(),
        "frenzied" => EmotionSignature::frenzied(),
        "ancient" => EmotionSignature::ancient(),
        // fuzzy/common synonyms in existing scrolls
        "clarity" => EmotionSignature {
            tone: "clear".into(),
            emphasis: 0.3,
            resonance: "focused".into(),
            intensity: Some(0.2),
        },
        "precision" => EmotionSignature {
            tone: "precise".into(),
            emphasis: 0.5,
            resonance: "crisp".into(),
            intensity: Some(0.4),
        },
        "awe" => EmotionSignature {
            tone: "awed".into(),
            emphasis: 0.7,
            resonance: "vast".into(),
            intensity: Some(0.6),
        },
        "fluidity" => EmotionSignature {
            tone: "fluid".into(),
            emphasis: 0.5,
            resonance: "flowing".into(),
            intensity: Some(0.5),
        },
        "purpose" => EmotionSignature {
            tone: "purposeful".into(),
            emphasis: 0.6,
            resonance: "directed".into(),
            intensity: Some(0.5),
        },
        _ => EmotionSignature::neutral(),
    }
}

impl<'de> Deserialize<'de> for EmotionSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = EmotionSigRepr::deserialize(deserializer)?;
        Ok(match repr {
            EmotionSigRepr::Str(s) => preset_from_str(&s),
            EmotionSigRepr::Obj {
                tone,
                emphasis,
                resonance,
                intensity,
            } => EmotionSignature {
                tone,
                emphasis,
                resonance,
                intensity,
            },
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YamlMetadata {
    pub title: String,
    pub scroll_type: ScrollType,
    pub emotion_signature: EmotionSignature,
    pub tags: Vec<String>,
    #[serde(default)]
    pub archetype: Option<String>,
    #[serde(default)]
    pub quorum_required: bool,
    #[serde(default)]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub file_path: Option<String>,
}
