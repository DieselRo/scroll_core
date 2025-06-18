//=========================================
//
//         src/core/context_frame_engine.rs
//
//=========================================



use crate::construct_ai::ConstructContext;
use crate::scroll::Scroll;
use crate::archive::archive_memory::InMemoryArchive;
use crate::archive::scroll_access_log::ScrollAccessLog;
use crate::schema::{ScrollType, ScrollStatus, YamlMetadata, EmotionSignature};

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
            ContextMode::Narrow => self.archive.query_by_tags(&triggering_scroll.yaml_metadata.tags),
            ContextMode::Broad => self.archive.query_by_emotion(triggering_scroll.emotion_signature.clone()),
            ContextMode::Echo => self.archive.query_by_links(&triggering_scroll.id),
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
