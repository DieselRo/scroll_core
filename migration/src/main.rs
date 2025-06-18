// migration/src/main.rs
use migration::{Migrator};
use sea_orm_migration::cli::run_cli;

// Add this:
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load from .env
    run_cli(Migrator).await;
}