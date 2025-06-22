//! Mockscribe is a lightweight construct used in tests and examples.
//! It simply echoes input and helps validate the invocation pipeline.
//! See [Mockscribe](../../../AGENTS.md#mockscribe) for context.
// src/invocation/constructs/mockscribe.rs

use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};

pub struct Mockscribe;

impl ConstructAI for Mockscribe {
    fn reflect_on_scroll(&self, context: &ConstructContext) -> ConstructResult {
        let input = context.user_input.as_deref().unwrap_or("");
        let text = if input.trim() == "ping" {
            "pong".to_string()
        } else {
            format!("echo: {}", input)
        };
        ConstructResult::Insight { text }
    }

    fn suggest_scroll(&self, _context: &ConstructContext) -> ConstructResult {
        ConstructResult::Refusal {
            reason: "not implemented".into(),
            echo: None,
        }
    }

    fn perform_scroll_action(&self, _context: &ConstructContext) -> ConstructResult {
        ConstructResult::Refusal {
            reason: "not implemented".into(),
            echo: None,
        }
    }

    fn name(&self) -> &str {
        "Mockscribe"
    }
}
