use std::collections::HashMap;
use chrono::{DateTime, Duration, Utc};
use super::types::{NotificationEvent, NotificationKind};

#[derive(Default)]
pub struct RateLimiter {
    // key: (thread_id, kind)
    last_sent: HashMap<(String, NotificationKind), DateTime<Utc>>,
}

impl RateLimiter {
    pub fn new() -> Self { Self { last_sent: HashMap::new() } }

    pub fn allow(&mut self, event: &NotificationEvent, minutes: u64) -> bool {
        let key = (event.thread_id.clone(), event.kind);
        let now = Utc::now();
        if let Some(prev) = self.last_sent.get(&key) {
            if now.signed_duration_since(*prev) < Duration::minutes(minutes as i64) {
                return false;
            }
        }
        self.last_sent.insert(key, now);
        true
    }
}

#[derive(Default)]
pub struct FlapSuppressor {
    // track last status change timestamp per thread
    last_status_change: HashMap<String, DateTime<Utc>>,
}

impl FlapSuppressor {
    pub fn record_status_change(&mut self, thread_id: &str) {
        self.last_status_change.insert(thread_id.to_string(), Utc::now());
    }

    pub fn suppress(&self, thread_id: &str, window_minutes: u64) -> bool {
        if let Some(ts) = self.last_status_change.get(thread_id) {
            return Utc::now().signed_duration_since(*ts) < Duration::minutes(window_minutes as i64);
        }
        false
    }
}

pub fn should_notify(event: &NotificationEvent) -> bool {
    match event.kind {
        NotificationKind::ThreadCreated => true,
        NotificationKind::AssignmentChanged => true,
        NotificationKind::StatusChanged => true,
        NotificationKind::Overdue => true,
    }
}


