//! Mythscribe is the Archive's primary language model construct.
//! It interprets invocations and produces poetic responses using the OpenAI client.
//! Mythscribe is typically routed by the InvocationManager after Aelren frames the context.
//! See [Mythscribe](../../../AGENTS.md#mythscribe) for a high level overview.
// src/invocation/constructs/mythscribe.rs
use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use crate::invocation::constructs::openai_construct::Mythscribe;
use crate::invocation::named_construct::NamedConstruct;
use crate::invocation::types::{Invocation, InvocationMode, InvocationResult};
use crate::schema::EmotionSignature;
use crate::scroll::Scroll;

impl Mythscribe {
    fn poetic_analyze(&self, input: &str) -> String {
        format!("{}? A curious verse indeed.", input)
    }
}

impl NamedConstruct for Mythscribe {
    fn name(&self) -> &str {
        "mythscribe"
    }

    fn perform(
        &self,
        invocation: &Invocation,
        scroll: Option<Scroll>,
    ) -> Result<InvocationResult, String> {
        let scroll = match scroll {
            Some(s) => s,
            None => {
                if !invocation.phrase.trim().is_empty() {
                    let text = self.poetic_analyze(&invocation.phrase);
                    return Ok(InvocationResult::Success(text.into_boxed_str()));
                } else {
                    return Err("No scroll provided".into());
                }
            }
        };
        let context = ConstructContext {
            scrolls: vec![scroll.clone()],
            emotion_signature: EmotionSignature::neutral(),
            tags: scroll.yaml_metadata.tags.clone(),
            user_input: Some(invocation.phrase.clone()),
        };

        let result = match invocation.mode {
            InvocationMode::Read => self.reflect_on_scroll(&context),
            InvocationMode::Modify => self.perform_scroll_action(&context),
            InvocationMode::Validate => self.reflect_on_scroll(&context),
            InvocationMode::Custom(_) | InvocationMode::Transition => ConstructResult::Refusal {
                reason: "Unsupported mode".into(),
                echo: None,
            },
        };

        let out = match result {
            ConstructResult::Insight { text } => InvocationResult::Success(text.into_boxed_str()),
            ConstructResult::ScrollDraft { content, .. } => {
                InvocationResult::Success(content.into_boxed_str())
            }
            ConstructResult::ModifiedScroll(s) => InvocationResult::ModifiedScroll(s),
            ConstructResult::Refusal { reason, echo } => {
                InvocationResult::Failure(echo.unwrap_or(reason).into_boxed_str())
            }
        };
        Ok(out)
    }
}
