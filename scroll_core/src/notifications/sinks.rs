use super::types::NotificationEvent;
use anyhow::Result;

pub trait Notifier: Send + Sync {
    fn send(&self, event: &NotificationEvent) -> Result<()>;
}

pub struct StdoutNotifier;
impl Notifier for StdoutNotifier {
    fn send(&self, event: &NotificationEvent) -> Result<()> {
        println!(
            "[notify] {:?} | {} | {} | {} | reason={}",
            event.kind,
            event.thread_id,
            event.title,
            event.scroll_path,
            event.reason.clone().unwrap_or_default()
        );
        Ok(())
    }
}

pub struct LogNotifier;
impl Notifier for LogNotifier {
    fn send(&self, event: &NotificationEvent) -> Result<()> {
        tracing::info!(
            target: "notify",
            kind = ?event.kind,
            thread_id = %event.thread_id,
            title = %event.title,
            scroll = %event.scroll_path,
            status = ?event.status,
            assignee = ?event.assignee,
            priority = ?event.priority,
            reason = ?event.reason,
            "notification"
        );
        Ok(())
    }
}

#[cfg(feature = "notifier_slack")]
pub struct SlackNotifier {
    webhook: String,
}

#[cfg(feature = "notifier_slack")]
impl SlackNotifier {
    pub fn from_env() -> Option<Self> {
        std::env::var("SC_SLACK_WEBHOOK_URL")
            .ok()
            .map(|wh| Self { webhook: wh })
    }
}

#[cfg(feature = "notifier_slack")]
impl Notifier for SlackNotifier {
    fn send(&self, event: &NotificationEvent) -> Result<()> {
        let text = format!(
            "[{kind:?}] {title} ({id})\n{path}\nreason: {reason}\nlink: <todo>",
            kind = event.kind,
            title = event.title,
            id = event.thread_id,
            path = event.scroll_path,
            reason = event.reason.clone().unwrap_or_default()
        );
        let payload = serde_json::json!({ "text": text });
        let client = reqwest::blocking::Client::new();
        let _ = client.post(&self.webhook).json(&payload).send()?;
        Ok(())
    }
}
