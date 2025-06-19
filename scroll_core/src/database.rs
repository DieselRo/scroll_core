// scroll_core/scroll_core/src/database.rs
// ================================================
// Database connection setup using SeaORM + SQLite
// ================================================

use sea_orm::{Database, DbConn, DbErr};
use std::env;
use std::path::Path;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    // Manually load from .env just in case
    dotenvy::from_filename("../.env").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("🧭 CARGO_MANIFEST_DIR: {}", env!("CARGO_MANIFEST_DIR"));

    let current_dir = std::env::current_dir().unwrap();
    println!("📂 Current working dir: {}", current_dir.display());
    println!("📎 DATABASE_URL: {}", database_url);

    if let Some(stripped) = database_url.strip_prefix("sqlite:") {
        let cleaned = stripped.trim_start_matches('/');
        println!("🔍 Checking if file exists: {cleaned}");
        println!(
            "🔍 Absolute path would be: {}",
            current_dir.join(cleaned).display()
        );

        if Path::new(cleaned).exists() {
            println!("✅ File found.");
        } else {
            println!("❌ File NOT found.");
        }
    }

    Database::connect(&database_url).await
}
