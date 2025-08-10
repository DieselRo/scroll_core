use migration::{Migrator, MigratorTrait};
use scroll_core::threads::dedupe_service::DedupeService;
use scroll_core::threads::thread_state_service::ThreadStateService;
use scroll_core::threads::types::{normalize_tags, Priority, ThreadStatus};
use sea_orm::Database;

#[tokio::test(flavor = "multi_thread")]
async fn transitions_and_reopen_count() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    let svc = ThreadStateService::new(&db);

    let t = svc
        .create(
            "scrolls/demo.md",
            "Transitions",
            None,
            Priority::Medium,
            None,
            None,
            Some("test"),
            "tester",
        )
        .await
        .unwrap();
    // Close it
    let t = svc
        .update_status(&t.id, ThreadStatus::Closed, Some("done"), "tester")
        .await
        .unwrap();
    assert_eq!(t.status.as_str(), "CLOSED");
    assert_eq!(t.reopened_count, 0);

    // Illegal transition Closed -> InProgress
    let err = svc
        .update_status(&t.id, ThreadStatus::InProgress, None, "tester")
        .await
        .unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("illegal transition"));

    // Reopen Closed -> Open increments count
    let t = svc
        .update_status(&t.id, ThreadStatus::Open, Some("revisit"), "tester")
        .await
        .unwrap();
    assert_eq!(t.status.as_str(), "OPEN");
    assert_eq!(t.reopened_count, 1);
}

#[test]
fn tag_normalization() {
    let out = normalize_tags(["Bug", "bug", "URGENT", "needs-Review", "urgent"]);
    assert_eq!(out, vec!["bug", "needs-review", "urgent"].into_iter().map(|s| s.to_string()).collect::<Vec<_>>());
}

#[tokio::test(flavor = "multi_thread")]
async fn dedupe_logic() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let dd = DedupeService::new(&db);
    let key = "scrolls/a.md|v1";
    let id1 = dd
        .dedupe_or_open(
            "scrolls/a.md",
            key,
            "Title",
            None,
            Priority::Low,
            Some(vec!["Bug".into(), "bug".into()]),
            None,
            Some("unit"),
            "tester",
        )
        .await
        .unwrap();
    // Second call with same key returns same id
    let id2 = dd
        .dedupe_or_open(
            "scrolls/a.md",
            key,
            "Title",
            None,
            Priority::Low,
            None,
            None,
            Some("unit"),
            "tester",
        )
        .await
        .unwrap();
    assert_eq!(id1, id2);
}

