//==========================================
// construct_ai.rs – Dreaming Constructs
//==========================================

//! Tools for building "Construct" AI personalities.
//!
//! A construct is an autonomous agent that can analyze and draft `Scroll`s.
//! Implement the [`ConstructAI`] trait on your type to plug it into the
//! Scroll Core runtime.
//!
//! ```rust,no_run
//! use scroll_core::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
//!
//! struct Historian;
//!
//! impl ConstructAI for Historian {
//!     fn reflect_on_scroll(&self, ctx: &ConstructContext) -> ConstructResult {
//!         // inspect ctx.scrolls and return an insight
//!         ConstructResult::Refusal { reason: String::from("todo"), echo: None }
//!     }
//!     fn suggest_scroll(&self, _ctx: &ConstructContext) -> ConstructResult { todo!() }
//!     fn perform_scroll_action(&self, _ctx: &ConstructContext) -> ConstructResult { todo!() }
//!     fn name(&self) -> &str { "historian" }
//! }
//! ```

use crate::schema::EmotionSignature;
use crate::scroll::Scroll;
use uuid::Uuid;

pub enum ConstructStyle {
    Historian,
    Poet,
    Weaver,
    Analyst,
}

#[derive(Debug, Clone)]
pub struct ConstructContext {
    pub scrolls: Vec<Scroll>,
    pub emotion_signature: EmotionSignature,
    pub tags: Vec<String>,
    pub user_input: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ConstructResult {
    Insight {
        text: String,
    },
    ScrollDraft {
        title: String,
        content: String,
    },
    ModifiedScroll(Box<Scroll>),
    Refusal {
        reason: String,
        echo: Option<String>,
    },
}

pub struct ConstructInsight {
    pub summary: String,
    pub improvement_suggestions: Vec<String>,
    pub symbolic_echo: Option<String>,
}

pub struct InvocationTrace {
    pub invoked_by: String,
    pub trigger_context: Option<String>,
    pub trace_link: Option<Uuid>,
}

pub struct DreamingConstruct {
    pub name: String,
    pub style: ConstructStyle,
    pub core_emotion: EmotionSignature,
    pub enabled: bool,
    pub cadence: Option<String>,
}

pub trait ConstructAI {
    fn reflect_on_scroll(&self, context: &ConstructContext) -> ConstructResult;
    fn suggest_scroll(&self, context: &ConstructContext) -> ConstructResult;
    fn perform_scroll_action(&self, context: &ConstructContext) -> ConstructResult;
    fn name(&self) -> &str;

    fn on_rejection(&self, reason: &str, symbolic_echo: Option<String>) {
        println!("{} hesitates: {}", self.name(), reason);
        if let Some(echo) = symbolic_echo {
            println!("Symbolic echo: {}", echo);
        }
    }

    fn hesitation_signal(&self) -> Option<String> {
        None
    }
}
