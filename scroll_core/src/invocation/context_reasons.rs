use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ReasonCode {
    Included,
    IncludedFallback,
    SelfExclusion,
    LowRelevance,
    MaxItems,
    TokenBudget,
    Excluded,
}

impl ReasonCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReasonCode::Included => "Included",
            ReasonCode::IncludedFallback => "IncludedFallback",
            ReasonCode::SelfExclusion => "SelfExclusion",
            ReasonCode::LowRelevance => "LowRelevance",
            ReasonCode::MaxItems => "MaxItems",
            ReasonCode::TokenBudget => "TokenBudget",
            ReasonCode::Excluded => "Excluded",
        }
    }
}

impl Display for ReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
