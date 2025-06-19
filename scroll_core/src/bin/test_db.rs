use dotenvy::dotenv;
use scroll_core::database::establish_connection;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Check if DATABASE_URL is being loaded properly
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "DATABASE_URL not set".to_string());

    println!("DATABASE_URL: {}", database_url); // This will print the database URL from the .env file

    match establish_connection().await {
        Ok(_) => println!("✅ Database connection successful."),
        Err(e) => eprintln!("❌ Database connection failed: {:?}", e),
    }
}
