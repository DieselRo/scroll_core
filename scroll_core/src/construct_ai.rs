//==========================================
// construct_ai.rs – Dreaming Constructs
//==========================================



#![allow(dead_code)]
#![allow(unused_imports)]

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
    Success(String),
    ModifiedScroll(Scroll),
    Refusal { reason: String, echo: Option<String> },
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