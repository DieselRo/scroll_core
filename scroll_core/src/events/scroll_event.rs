// src/events/scroll_event.rs
// ===================================================
// Defines ScrollEvent: a discrete interaction or state change
// captured during invocation. Used in session timelines.
// ===================================================

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constructs::construct_metadata::ConstructName;
use crate::memory::memory_result::MemoryDelta;
use crate::models::base_model::LLMResponseContent;

/// A single symbolic event recorded in a session timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollEvent {
    /// Unique ID for the event
    pub id: Uuid,

    /// Which Construct or actor generated this event
    pub author: ConstructName,

    /// When this event occurred (Unix timestamp)
    pub timestamp: f64,

    /// Optional content such as a model response, text, or code
    pub content: Option<LLMResponseContent>,

    /// Optional memory delta / symbolic state change
    pub actions: Option<MemoryDelta>,

    /// If true, event is partial (e.g., streaming in progress)
    pub partial: bool,

    /// If true, Construct believes this turn is complete
    pub turn_complete: bool,

    /// If true, this event was interrupted (cut short)
    pub interrupted: bool,

    /// Optional branch label for diverging timelines
    pub branch: Option<String>,
}

impl ScrollEvent {
    /// Create a new ScrollEvent with auto-generated ID and timestamp
    pub fn new(
        author: ConstructName,
        content: Option<LLMResponseContent>,
        actions: Option<MemoryDelta>,
        partial: bool,
        turn_complete: bool,
        interrupted: bool,
        branch: Option<String>,
    ) -> Self {
        ScrollEvent {
            id: Uuid::new_v4(),
            author,
            timestamp: Utc::now().timestamp_millis() as f64 / 1000.0,
            content,
            actions,
            partial,
            turn_complete,
            interrupted,
            branch,
        }
    }
}
