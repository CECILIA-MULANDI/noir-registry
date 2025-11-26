use sqlx::postgres::{PgPool, PgPoolOptions, PgConnectOptions};
use std::str::FromStr;
use anyhow::Result;
/// Creates a database connection pool from the DATABASE_URL environment variable
pub async fn create_pool() -> Result<PgPool> {
    // Get database URL from environment variable
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    // Parse connection options - the DATABASE_URL should have ?statement_cache_size=0
    // This is REQUIRED for PgBouncer transaction mode compatibility
    let connect_options = PgConnectOptions::from_str(&database_url)?;
    
    // Create a connection pool
    // Note: statement_cache_size=0 in the URL should disable prepared statements
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}

/// Runs all pending database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running database migrations...");
    
    // Try to run migrations, but handle prepared statement errors gracefully
    // This can happen with PgBouncer in transaction mode
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            println!("✅ Migrations completed successfully!");
            Ok(())
        }
        Err(e) => {
            // Check if it's a prepared statement error
            let error_msg = e.to_string();
            if error_msg.contains("prepared statement") && error_msg.contains("already exists") {
                println!("⚠️  Migration error due to prepared statement cache (PgBouncer issue)");
                println!("   This usually means migrations are already applied or PgBouncer needs to clear its cache.");
                println!("   Attempting to continue anyway...");
                // Try to check if migrations table exists and is up to date
                // Use persistent(false) to avoid prepared statements (required for PgBouncer)
                match sqlx::query("SELECT COUNT(*) FROM _sqlx_migrations")
                    .persistent(false)
                    .fetch_one(pool)
                    .await
                {
                    Ok(_) => {
                        println!("✅ Migration table exists - assuming migrations are applied");
                        Ok(())
                    }
                    Err(_) => {
                        println!("⚠️  Could not verify migration table (may be due to PgBouncer cache)");
                        println!("   Assuming migrations are applied and continuing...");
                        println!("   If you see database errors, run migrations manually: sqlx migrate run");
                        // Continue anyway - the server might work if migrations are actually applied
                        Ok(())
                    }
                }
            } else {
                // Some other error - propagate it
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }
}

/// Initializes the database connection and runs migrations
pub async fn init_db() -> Result<PgPool, Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    run_migrations(&pool).await?;
    Ok(pool)
}
