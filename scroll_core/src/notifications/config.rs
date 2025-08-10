#[derive(Debug, Clone)]
pub struct NotificationsConfig {
    pub enabled: bool,
    pub sinks: Vec<String>,
    pub rate_limit_minutes: u64,
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sinks: vec!["stdout".into(), "log".into()],
            rate_limit_minutes: 10,
        }
    }
}

pub fn load_from_env() -> NotificationsConfig {
    let enabled = std::env::var("SC_NOTIFICATIONS_ENABLED")
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(true);
    let rate_limit_minutes = std::env::var("SC_NOTIFICATIONS_RATE_MIN")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(10);
    let sinks = std::env::var("SC_NOTIFICATIONS_SINKS")
        .ok()
        .map(|s| {
            s.split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect()
        })
        .unwrap_or_else(|| vec!["stdout".into(), "log".into()]);
    NotificationsConfig {
        enabled,
        sinks,
        rate_limit_minutes,
    }
}
