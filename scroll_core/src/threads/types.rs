use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThreadStatus {
    Open,
    InProgress,
    Blocked,
    Closed,
}

impl ThreadStatus {
    pub fn as_db(&self) -> &'static str {
        match self {
            ThreadStatus::Open => "OPEN",
            ThreadStatus::InProgress => "IN_PROGRESS",
            ThreadStatus::Blocked => "BLOCKED",
            ThreadStatus::Closed => "CLOSED",
        }
    }

    pub fn allowed_transition(from: ThreadStatus, to: ThreadStatus) -> bool {
        use ThreadStatus::*;
        match (from, to) {
            // No-op allowed
            (a, b) if a == b => true,
            // Open -> {InProgress, Blocked, Closed}
            (Open, InProgress | Blocked | Closed) => true,
            // InProgress -> {Open, Blocked, Closed}
            (InProgress, Open | Blocked | Closed) => true,
            // Blocked -> {Open, InProgress, Closed}
            (Blocked, Open | InProgress | Closed) => true,
            // Closed -> {Open} (reopen only)
            (Closed, Open) => true,
            _ => false,
        }
    }
}

impl Display for ThreadStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_db())
    }
}

impl FromStr for ThreadStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "OPEN" => Ok(ThreadStatus::Open),
            "IN_PROGRESS" | "INPROGRESS" => Ok(ThreadStatus::InProgress),
            "BLOCKED" => Ok(ThreadStatus::Blocked),
            "CLOSED" => Ok(ThreadStatus::Closed),
            _ => Err(format!(
                "invalid status: {} (allowed: OPEN, IN_PROGRESS, BLOCKED, CLOSED)",
                s
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn as_db(&self) -> &'static str {
        match self {
            Priority::Low => "LOW",
            Priority::Medium => "MEDIUM",
            Priority::High => "HIGH",
        }
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_db())
    }
}

impl FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "LOW" => Ok(Priority::Low),
            "MEDIUM" | "MED" => Ok(Priority::Medium),
            "HIGH" => Ok(Priority::High),
            _ => Err(format!("invalid priority: {} (allowed: LOW, MEDIUM, HIGH)", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThreadEventType {
    Comment,
    StatusChange,
    Assignment,
    TagChange,
    SystemNote,
}

impl ThreadEventType {
    pub fn as_db(&self) -> &'static str {
        match self {
            ThreadEventType::Comment => "COMMENT",
            ThreadEventType::StatusChange => "STATUS_CHANGE",
            ThreadEventType::Assignment => "ASSIGNMENT",
            ThreadEventType::TagChange => "TAG_CHANGE",
            ThreadEventType::SystemNote => "SYSTEM_NOTE",
        }
    }
}

impl Display for ThreadEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_db())
    }
}

impl FromStr for ThreadEventType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "COMMENT" => Ok(ThreadEventType::Comment),
            "STATUS_CHANGE" | "STATUS" => Ok(ThreadEventType::StatusChange),
            "ASSIGNMENT" | "ASSIGN" => Ok(ThreadEventType::Assignment),
            "TAG_CHANGE" | "TAGS" => Ok(ThreadEventType::TagChange),
            "SYSTEM_NOTE" | "SYSTEM" => Ok(ThreadEventType::SystemNote),
            _ => Err(format!(
                "invalid event type: {} (COMMENT, STATUS_CHANGE, ASSIGNMENT, TAG_CHANGE, SYSTEM_NOTE)",
                s
            )),
        }
    }
}

pub fn normalize_tags<T: AsRef<str>>(tags: impl IntoIterator<Item = T>) -> Vec<String> {
    let mut v: Vec<String> = tags
        .into_iter()
        .map(|t| t.as_ref().trim().to_ascii_lowercase())
        .filter(|t| !t.is_empty())
        .collect();
    v.sort();
    v.dedup();
    v
}

pub fn tags_to_db(tags: &[String]) -> String {
    tags.join(",")
}

pub fn tags_from_db(s: &str) -> Vec<String> {
    if s.trim().is_empty() {
        return vec![];
    }
    s.split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

