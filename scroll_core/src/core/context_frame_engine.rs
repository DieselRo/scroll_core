//=========================================
//
//         src/core/context_frame_engine.rs
//
//=========================================

use crate::archive::archive_memory::InMemoryArchive;
use crate::archive::scroll_access_log::ScrollAccessLog;
use crate::construct_ai::ConstructContext;
use crate::scroll::Scroll;

pub enum ContextMode {
    Narrow,
    Broad,
    Echo,
}

pub struct ContextFrameEngine<'a> {
    pub archive: &'a InMemoryArchive,
    pub access_log: Option<&'a ScrollAccessLog>,
    pub mode: ContextMode,
    pub max_scrolls: usize,
}

impl<'a> ContextFrameEngine<'a> {
    pub fn new(archive: &'a InMemoryArchive, mode: ContextMode) -> Self {
        Self {
            archive,
            access_log: None,
            mode,
            max_scrolls: 5,
        }
    }

    pub fn with_access_log(mut self, log: &'a ScrollAccessLog) -> Self {
        self.access_log = Some(log);
        self
    }

    pub fn build_context(&self, triggering_scroll: &Scroll) -> ConstructContext {
        let mut scrolls = vec![triggering_scroll.clone()];

        let related = match self.mode {
            ContextMode::Narrow => self
                .archive
                .query_by_tags(&triggering_scroll.yaml_metadata.tags),
            ContextMode::Broad => {
                let query = compose_query(triggering_scroll);
                let semantic: Vec<_> = self
                    .archive
                    .query_semantic(&query, self.max_scrolls * 2)
                    .into_iter()
                    .filter(|(_, score)| *score > 0.65)
                    .map(|(s, _)| s)
                    .collect();
                if semantic.is_empty() {
                    self.archive
                        .query_by_emotion(triggering_scroll.emotion_signature.clone())
                } else {
                    semantic
                }
            }
            ContextMode::Echo => {
                let query = compose_query(triggering_scroll);
                let semantic: Vec<_> = self
                    .archive
                    .query_semantic(&query, self.max_scrolls * 2)
                    .into_iter()
                    .filter(|(_, score)| *score > 0.65)
                    .map(|(s, _)| s)
                    .collect();
                if semantic.is_empty() {
                    self.archive.query_by_links(&triggering_scroll.id)
                } else {
                    semantic
                }
            }
        };

        for s in related {
            if scrolls.len() >= self.max_scrolls {
                break;
            }
            if s.id != triggering_scroll.id {
                scrolls.push(s);
            }
        }

        ConstructContext {
            scrolls,
            emotion_signature: triggering_scroll.emotion_signature.clone(),
            tags: triggering_scroll.yaml_metadata.tags.clone(),
            user_input: None,
        }
    }
}

fn compose_query(scroll: &Scroll) -> String {
    let first_lines = scroll
        .markdown_body
        .lines()
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "{} {} {}",
        scroll.title,
        scroll.yaml_metadata.tags.join(" "),
        first_lines
    )
}
