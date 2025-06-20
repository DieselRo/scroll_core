// migration/src/main.rs
#![warn(unused_imports)]
use migration::Migrator;
#[cfg(feature = "cli")]
use sea_orm_migration::cli::run_cli;

// Add this:
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load from .env
    #[cfg(feature = "cli")]
    run_cli(Migrator).await;
}
