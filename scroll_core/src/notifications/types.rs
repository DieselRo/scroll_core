use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotificationKind {
    ThreadCreated,
    AssignmentChanged,
    StatusChanged,
    Overdue,
}

#[derive(Debug, Clone)]
pub struct NotificationEvent {
    pub kind: NotificationKind,
    pub thread_id: String,
    pub title: String,
    pub scroll_path: String,
    pub status: Option<String>,
    pub assignee: Option<String>,
    pub priority: Option<String>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl NotificationEvent {
    pub fn new(kind: NotificationKind, thread_id: String, title: String, scroll_path: String) -> Self {
        Self {
            kind,
            thread_id,
            title,
            scroll_path,
            status: None,
            assignee: None,
            priority: None,
            reason: None,
            created_at: Utc::now(),
        }
    }
}


