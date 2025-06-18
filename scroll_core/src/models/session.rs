// models/session.rs

use chrono::{DateTime, Utc};
use serde_json::{Map as JsonMap, Value};
use std::collections::HashMap;

use crate::entities::session_record::Model as SessionRecord;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub state: HashMap<String, Value>,
}

// =============================
// 🔁 Conversion Implementations
// =============================

impl From<SessionRecord> for Session {
    fn from(record: SessionRecord) -> Self {
        // Convert JSON object into a HashMap
        let state_map = match record.state {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(), // fallback: empty state if not an object
        };

        Session {
            id: record.id,
            user_id: record.user_id,
            created_at: record.created_at,
            state: state_map,
        }
    }
}

impl From<Session> for SessionRecord {
    fn from(session: Session) -> Self {
        let state_json = Value::Object(session.state.into_iter().collect());

        SessionRecord {
            id: session.id,
            user_id: session.user_id,
            created_at: session.created_at,
            state: state_json,
        }
    }
}
