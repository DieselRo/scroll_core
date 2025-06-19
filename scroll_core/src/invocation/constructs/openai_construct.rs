//===================================
// src/invocation/constructs/openai_construct.rs
//====================================

use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use reqwest::blocking::Client;
use dotenvy::dotenv;
use std::env;



// === OpenAI Client & Config ===
#[derive(Debug, Clone)]
pub struct OpenAIClient {
    pub api_key: String,
    pub model: String,
    pub endpoint: String,
    pub max_tokens: usize,
}

impl OpenAIClient {
    pub fn new_from_env() -> Self {
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set in .env or environment");

        Self {
            api_key,
            model: "gpt-4o".to_string(),
            endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            max_tokens: 750,
        }
    }

    pub fn send_prompt(&self, prompt: &str) -> Result<String, String> {
        let client = Client::new();

        let body = serde_json::json!({
            "model": self.model,
            "messages": [{"role": "system", "content": prompt}],
            "max_tokens": self.max_tokens
        });

        let res = client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .map_err(|e| format!("HTTP error: {}", e))?;

        let json: serde_json::Value = res.json().map_err(|e| format!("JSON parse error: {}", e))?;
        let response_text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Malformed response")?
            .to_string();

        Ok(response_text)
    }
}

// === Mythscribe Construct ===
pub struct Mythscribe {
    pub client: OpenAIClient,
    pub system_prompt: String,
}

impl Mythscribe {
    pub fn new(client: OpenAIClient, system_prompt: String) -> Self {
        Self { client, system_prompt }
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
                scroll.title,
                scroll.yaml_metadata.tags,
                scroll.markdown_body,
            ));
        }

        let full_prompt = format!(
            "{}\n\nCONTEXT:\n{}",
            self.system_prompt,
            prompt_sections.join("\n")
        );

        match self.client.send_prompt(&full_prompt) {
            Ok(response) => ConstructResult::Insight { text: response },
            Err(err) => ConstructResult::Refusal {
                reason: format!("Invocation failed: {}", err),
                echo: Some("The Archive stirred, but no voice replied.".to_string()),
            },
        }
    }

    fn suggest_scroll(&self, _context: &ConstructContext) -> ConstructResult {
        match self.client.send_prompt("Propose new scroll") {
            Ok(response) => ConstructResult::ScrollDraft {
                title: "Proposed Scroll".into(),
                content: response,
            },
            Err(err) => ConstructResult::Refusal {
                reason: format!("Invocation failed: {}", err),
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