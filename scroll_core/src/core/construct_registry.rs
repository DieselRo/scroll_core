//==========================
//      construct_registry.rs
//==========================

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use crate::scroll::Scroll;
use crate::schema::EmotionSignature;

pub struct ConstructRegistry {
    constructs: HashMap<String, Arc<dyn ConstructAI + Send + Sync>>, // thread-safe
}

impl ConstructRegistry {
    pub fn new() -> Self {
        Self {
            constructs: HashMap::new(),
        }
    }

    pub fn insert<T>(&mut self, name: &str, construct: T)
    where
        T: ConstructAI + Send + Sync + 'static,
    {
        self.constructs.insert(name.to_string(), Arc::new(construct));
    }

    pub fn invoke(&self, name: &str, context: &ConstructContext) -> ConstructResult {
        match self.constructs.get(name) {
            Some(construct) => construct.reflect_on_scroll(context),
            None => ConstructResult::Refusal {
                reason: format!("No Construct found with name '{}'.", name),
                echo: Some("The name was whispered, but no presence replied.".into()),
            },
        }
    }

    pub fn list_constructs(&self) -> Vec<String> {
        self.constructs.keys().cloned().collect()
    }

    pub fn build_context_from_scroll(&self, scroll: &Scroll, user_input: &str) -> ConstructContext {
        ConstructContext {
            scrolls: vec![scroll.clone()],
            emotion_signature: EmotionSignature::neutral(),
            tags: vec![],
            user_input: Some(user_input.to_string()),
        }
    }
}
