// src/invocation/constructs/mythscribe.rs
use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use crate::invocation::constructs::openai_construct::Mythscribe;
use crate::invocation::invocation_core::{Invocation, InvocationMode, InvocationResult};
use crate::invocation::named_construct::NamedConstruct;
use crate::schema::EmotionSignature;
use crate::scroll::Scroll;

impl NamedConstruct for Mythscribe {
    fn name(&self) -> &str {
        "mythscribe"
    }

    fn perform(
        &self,
        invocation: &Invocation,
        scroll: Option<Scroll>,
    ) -> Result<InvocationResult, String> {
        let scroll = scroll.ok_or("No scroll provided")?;
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
            ConstructResult::Insight { text } => InvocationResult::Success(text),
            ConstructResult::ScrollDraft { content, .. } => InvocationResult::Success(content),
            ConstructResult::ModifiedScroll(s) => InvocationResult::ModifiedScroll(s),
            ConstructResult::Refusal { reason, echo } => {
                InvocationResult::Failure(echo.unwrap_or(reason))
            }
        };
        Ok(out)
    }
}
