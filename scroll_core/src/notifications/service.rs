use super::config::{load_from_env, NotificationsConfig};
use super::policy::{should_notify, FlapSuppressor, RateLimiter};
use super::sinks::{LogNotifier, Notifier, StdoutNotifier};
use super::types::{NotificationEvent, NotificationKind};
use anyhow::Result;
use std::sync::{Arc, Mutex};

#[cfg(feature = "notifier_slack")]
use super::sinks::SlackNotifier;

pub struct NotificationHub {
    config: NotificationsConfig,
    sinks: Vec<Box<dyn Notifier>>,
    rate: Mutex<RateLimiter>,
    flap: Mutex<FlapSuppressor>,
}

impl NotificationHub {
    pub fn from_env() -> Self {
        let cfg = load_from_env();
        let mut sinks: Vec<Box<dyn Notifier>> = Vec::new();
        for s in &cfg.sinks {
            match s.as_str() {
                "stdout" => sinks.push(Box::new(StdoutNotifier)),
                "log" => sinks.push(Box::new(LogNotifier)),
                #[cfg(feature = "notifier_slack")]
                "slack" => {
                    if let Some(sn) = SlackNotifier::from_env() {
                        sinks.push(Box::new(sn));
                    }
                }
                _ => {}
            }
        }
        Self {
            config: cfg,
            sinks,
            rate: Mutex::new(RateLimiter::new()),
            flap: Mutex::new(FlapSuppressor::default()),
        }
    }

    pub fn dispatch(&self, mut event: NotificationEvent) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }
        if !should_notify(&event) {
            return Ok(());
        }

        // Flap control for status changes
        if matches!(event.kind, NotificationKind::StatusChanged) {
            if self.flap.lock().unwrap().suppress(&event.thread_id, 5) {
                return Ok(());
            }
            self.flap
                .lock()
                .unwrap()
                .record_status_change(&event.thread_id);
        }

        // Rate limit per thread per kind
        if !self
            .rate
            .lock()
            .unwrap()
            .allow(&event, self.config.rate_limit_minutes)
        {
            return Ok(());
        }

        // Ensure a concise template
        if event.reason.is_none() {
            event.reason = Some(String::new());
        }
        for sink in &self.sinks {
            let _ = sink.send(&event);
        }
        Ok(())
    }
}

// Global hub (lazy)
use once_cell::sync::OnceCell;
static HUB: OnceCell<Arc<NotificationHub>> = OnceCell::new();

pub fn get_hub() -> Arc<NotificationHub> {
    HUB.get_or_init(|| Arc::new(NotificationHub::from_env()))
        .clone()
}

pub fn notify_event(event: NotificationEvent) -> Result<()> {
    get_hub().dispatch(event)
}

// Helper for overdue notification from nudge logic
pub fn notify_overdue_thread(
    thread_id: &str,
    title: &str,
    scroll_path: &str,
    reason: Option<&str>,
) -> Result<()> {
    let mut ev = NotificationEvent::new(
        NotificationKind::Overdue,
        thread_id.to_string(),
        title.to_string(),
        scroll_path.to_string(),
    );
    ev.reason = reason.map(|s| s.to_string());
    notify_event(ev)
}
