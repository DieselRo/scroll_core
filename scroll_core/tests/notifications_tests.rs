use scroll_core::notifications::{NotificationEvent, NotificationKind};

#[test]
fn policy_should_allow_expected_kinds() {
    let ev = NotificationEvent::new(
        NotificationKind::ThreadCreated,
        "id1".into(),
        "Title".into(),
        "scrolls/a.md".into(),
    );
    assert!(scroll_core::notifications::policy::should_notify(&ev));
    let ev2 = NotificationEvent::new(NotificationKind::Overdue, "id1".into(), "Title".into(), "scrolls/a.md".into());
    assert!(scroll_core::notifications::policy::should_notify(&ev2));
}

#[test]
fn rate_limiter_blocks_within_window() {
    let mut rl = scroll_core::notifications::policy::RateLimiter::new();
    let ev = NotificationEvent::new(NotificationKind::Overdue, "t1".into(), "T".into(), "p".into());
    assert!(rl.allow(&ev, 10));
    assert!(!rl.allow(&ev, 10));
}


