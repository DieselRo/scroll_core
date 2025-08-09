//! Mythscribe powered by a provider-agnostic async LLM client.
//! This replaces the prior blocking OpenAI-only implementation.
//===================================
// src/invocation/constructs/openai_construct.rs
//====================================

use std::sync::Arc;

use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use crate::invocation::llm::{util::send_blocking, LLMClient, LLMError};

// === Mythscribe Construct (LLM-backed) ===
pub struct Mythscribe {
    pub client: Arc<dyn LLMClient + Send + Sync>,
    pub system_prompt: String,
}

impl Mythscribe {
    pub fn new(client: Arc<dyn LLMClient + Send + Sync>, system_prompt: String) -> Self {
        Self {
            client,
            system_prompt,
        }
    }
}

impl ConstructAI for Mythscribe {
    fn reflect_on_scroll(&self, context: &ConstructContext) -> ConstructResult {
        if context.scrolls.is_empty() {
            return ConstructResult::Refusal {
                reason: "No scrolls provided to reflect on.".into(),
                echo: Some("The Archive held no memory to echo.".into()),
            };
        }

        let mut prompt_sections = vec![];
        for scroll in &context.scrolls {
            prompt_sections.push(format!(
                "Title: {}\nTags: {:?}\nBody:\n{}\n---\n",
                scroll.title, scroll.yaml_metadata.tags, scroll.markdown_body,
            ));
        }
        let full_prompt = format!(
            "{}\n\nCONTEXT:\n{}",
            self.system_prompt,
            prompt_sections.join("\n")
        );

        match send_blocking(self.client.as_ref(), &full_prompt) {
            Ok(response) => ConstructResult::Insight { text: response },
            Err(err) => ConstructResult::Refusal {
                reason: format!("Invocation failed: {}", map_err(&err)),
                echo: Some("The Archive stirred, but no voice replied.".to_string()),
            },
        }
    }

    fn suggest_scroll(&self, _context: &ConstructContext) -> ConstructResult {
        match send_blocking(
            self.client.as_ref(),
            "Propose a new scroll in one paragraph.",
        ) {
            Ok(response) => ConstructResult::ScrollDraft {
                title: "Proposed Scroll".into(),
                content: response,
            },
            Err(err) => ConstructResult::Refusal {
                reason: format!("Invocation failed: {}", map_err(&err)),
                echo: Some("The glyphs remain unwritten.".into()),
            },
        }
    }

    fn perform_scroll_action(&self, _context: &ConstructContext) -> ConstructResult {
        ConstructResult::Refusal {
            reason: "Mythscribe does not perform direct actions.".into(),
            echo: Some("It only speaks in echoes.".into()),
        }
    }

    fn name(&self) -> &str {
        "Mythscribe"
    }
}

fn map_err(e: &LLMError) -> String {
    match e {
        LLMError::Config(_) => "configuration error".into(),
        LLMError::Auth => "auth error".into(),
        LLMError::RateLimit => "rate limited".into(),
        LLMError::Timeout => "timeout".into(),
        LLMError::Transient(s)
        | LLMError::Network(s)
        | LLMError::Server(s)
        | LLMError::Other(s)
        | LLMError::Unsupported(s) => s.clone(),
    }
}
