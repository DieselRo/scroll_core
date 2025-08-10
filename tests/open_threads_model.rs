use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Database, EntityTrait, Set};

#[tokio::test(flavor = "multi_thread")]
async fn open_threads_crud() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect("sqlite::memory:").await?;
    Migrator::up(&db, None).await?;

    // Create
    let now = chrono::Utc::now();
    let rec = scroll_core::entities::open_threads::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        scroll_path: Set("scrolls/demo.md".into()),
        title: Set("Test thread".into()),
        status: Set("OPEN".into()),
        assignee: Set(None),
        priority: Set("MEDIUM".into()),
        tags: Set(None),
        due_at: Set(None),
        source: Set(None),
        reopened_count: Set(0),
        dedupe_key: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        last_event_id: Set(None),
    }
    .insert(&db)
    .await?;

    // Read
    let got = scroll_core::entities::open_threads::Entity::find_by_id(rec.id.clone())
        .one(&db)
        .await?
        .expect("record present");
    assert_eq!(got.status, "OPEN");

    // Update
    let mut upd = got.clone().into_active_model();
    upd.status = Set("CLOSED".into());
    let got2 = upd.update(&db).await?;
    assert_eq!(got2.status, "CLOSED");

    Ok(())
}
