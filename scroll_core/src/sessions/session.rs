//! Defines ScrollSession which stores conversation history and metadata.
//! Sessions keep a log of events and track the user's active state.
//! See [Sessions](../../AGENTS.md#contextframeengine) for design rationale.
// sessions/session.rs
// ===================================================
// Defines ScrollSession, a central unit for dialogue
// session state across invocations.
// ===================================================

use crate::events::scroll_event::ScrollEvent;
use crate::sessions::state::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollSession {
    /// Unique identifier of the session
    pub id: String,

    /// Name of the application invoking the session
    pub app_name: String,

    /// The user to whom the session belongs
    pub user_id: String,

    /// Current scroll state (merged from app/user/session)
    pub state: State,

    /// Chronological sequence of events (user/construct/agent/etc)
    pub events: Vec<ScrollEvent>,

    /// Last update timestamp (seconds since epoch)
    pub last_update_time: u64,
}

impl ScrollSession {
    pub fn new(
        id: String,
        app_name: String,
        user_id: String,
        initial_state: HashMap<String, String>,
    ) -> Self {
        Self {
            id,
            app_name,
            user_id,
            events: vec![],
            state: State::from_parts(initial_state, HashMap::new()),
            last_update_time: chrono::Utc::now().timestamp() as u64,
        }
    }
}
