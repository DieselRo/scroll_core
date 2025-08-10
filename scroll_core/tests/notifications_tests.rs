use scroll_core::notifications::policy::{should_notify, RateLimiter};
use scroll_core::notifications::types::{NotificationEvent, NotificationKind};

#[test]
fn policy_allows_kinds() {
    let ev = NotificationEvent::new(
        NotificationKind::ThreadCreated,
        "t1".into(),
        "a".into(),
        "s".into(),
    );
    assert!(should_notify(&ev));
    let ev2 = NotificationEvent::new(
        NotificationKind::AssignmentChanged,
        "t1".into(),
        "a".into(),
        "s".into(),
    );
    assert!(should_notify(&ev2));
    let ev3 = NotificationEvent::new(
        NotificationKind::StatusChanged,
        "t1".into(),
        "a".into(),
        "s".into(),
    );
    assert!(should_notify(&ev3));
    let ev4 = NotificationEvent::new(
        NotificationKind::Overdue,
        "t1".into(),
        "a".into(),
        "s".into(),
    );
    assert!(should_notify(&ev4));
}

#[test]
fn rate_limiter_blocks_duplicates() {
    let mut rl = RateLimiter::new();
    let ev = NotificationEvent::new(
        NotificationKind::ThreadCreated,
        "t1".into(),
        "a".into(),
        "s".into(),
    );
    assert!(rl.allow(&ev, 10));
    assert!(!rl.allow(&ev, 10));
}
