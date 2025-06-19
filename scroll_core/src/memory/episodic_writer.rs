use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::scroll_writer::ScrollWriter;
use crate::sessions::session::ScrollSession;
use crate::sessions::session_service::SessionService;
use crate::schema::{EmotionSignature, ScrollStatus, ScrollType, YamlMetadata};
use crate::scroll::{Scroll, ScrollOrigin};

pub struct EpisodicWriterJob {
    service: Arc<dyn SessionService + Send + Sync>,
    app_name: String,
    user_id: String,
    token_threshold: usize,
    base_path: PathBuf,
    last_run: Mutex<DateTime<Utc>>, // tracks last execution
}

impl EpisodicWriterJob {
    pub fn new(
        service: Arc<dyn SessionService + Send + Sync>,
        app_name: String,
        user_id: String,
        token_threshold: usize,
        base_path: PathBuf,
    ) -> Self {
        Self {
            service,
            app_name,
            user_id,
            token_threshold,
            base_path,
            last_run: Mutex::new(Utc::now() - chrono::Duration::days(1)),
        }
    }

    fn summarize_session(&self, session: &ScrollSession) -> (String, Vec<String>, Vec<String>) {
        let mut participants: HashSet<String> = HashSet::new();
        let mut token_count = 0usize;
        for ev in &session.events {
            participants.insert(ev.author.clone());
            if let Some(content) = &ev.content {
                token_count += content.text.split_whitespace().count();
            }
        }
        let parts: Vec<String> = participants.iter().cloned().collect();
        let summary = format!(
            "Conversation with {} events ({} tokens) between {}.",
            session.events.len(),
            token_count,
            parts.join(", ")
        );
        (summary, parts.clone(), parts)
    }

    fn write_scroll(&self, summary: &str, tags: &[String]) -> Result<(), String> {
        let now = Utc::now();
        let dir = &self.base_path;
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        let file_path = dir.join(format!("episodic-{}.md", now.format("%Y%m%d")));

        let metadata = YamlMetadata {
            title: format!("Episodic Memory {}", now.format("%Y-%m-%d")),
            scroll_type: ScrollType::Echo,
            emotion_signature: EmotionSignature::reflective(),
            tags: tags.to_vec(),
            last_modified: Some(now),
            file_path: Some(file_path.to_string_lossy().into()),
        };

        let scroll = Scroll {
            id: Uuid::new_v4(),
            title: metadata.title.clone(),
            scroll_type: ScrollType::Echo,
            yaml_metadata: metadata,
            markdown_body: summary.to_string(),
            invocation_phrase: String::new(),
            sigil: String::new(),
            status: ScrollStatus::Draft,
            emotion_signature: EmotionSignature::reflective(),
            linked_scrolls: Vec::new(),
            origin: ScrollOrigin {
                created: now,
                authored_by: None,
                last_modified: now,
            },
        };

        ScrollWriter::write_scroll(&scroll, &file_path)
    }

    pub async fn run_once(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self
            .service
            .list_sessions(&self.app_name, &self.user_id)
            .await?;
        for mut session in resp.sessions {
            let token_count: usize = session
                .events
                .iter()
                .map(|e| e.content.as_ref().map_or(0, |c| c.text.split_whitespace().count()))
                .sum();
            if token_count > self.token_threshold {
                let (summary, _participants, tags) = self.summarize_session(&session);
                self.write_scroll(&summary, &tags)?;
            }
        }
        Ok(())
    }
}

impl crate::invocation::named_construct::PulseSensitive for EpisodicWriterJob {
    fn should_awaken(&self, _tick: u64) -> bool {
        let last = *self.last_run.lock().unwrap();
        Utc::now().signed_duration_since(last).num_hours() >= 24
    }
}

impl crate::invocation::named_construct::NamedConstruct for EpisodicWriterJob {
    fn name(&self) -> &str {
        "episodic_writer"
    }

    fn perform(
        &self,
        _invocation: &crate::invocation::invocation::Invocation,
        _scroll: Option<crate::Scroll>,
    ) -> Result<crate::invocation::invocation::InvocationResult, String> {
        let rt = Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(self.run_once()).map_err(|e| e.to_string())?;
        *self.last_run.lock().unwrap() = Utc::now();
        Ok(crate::invocation::invocation::InvocationResult::Success(
            "episodic written".into(),
        ))
    }

    fn as_pulse_sensitive(&self) -> Option<&dyn crate::invocation::named_construct::PulseSensitive> {
        Some(self)
    }
}
