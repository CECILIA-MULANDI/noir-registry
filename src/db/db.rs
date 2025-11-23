use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::Result;
/// Creates a database connection pool from the DATABASE_URL environment variable
pub async fn create_pool() -> Result<PgPool> {
    // Get database URL from environment variable
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

/// Runs all pending database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running database migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;

    println!("✅ Migrations completed successfully!");
    Ok(())
}

/// Initializes the database connection and runs migrations
pub async fn init_db() -> Result<PgPool, Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    run_migrations(&pool).await?;
    Ok(pool)
}
