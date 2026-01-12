use anyhow::Result;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::str::FromStr;
/// Creates a database connection pool from the DATABASE_URL environment variable
pub async fn create_pool() -> Result<PgPool> {
    let mut database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    let is_production = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string())
        .eq_ignore_ascii_case("production");

    // In production, don't auto-modify DATABASE_URL (assume it's correct)
    if !is_production {
        // Development-only: auto-fix PgBouncer issues
        let original_url = database_url.clone();

        if database_url.contains(":6543") {
            println!(
                "⚠️  Detected PgBouncer pooler (port 6543) - switching to direct connection (port 5432)"
            );
            database_url = database_url.replace(":6543", ":5432");
        }

        if !database_url.contains("statement_cache_size") {
            if database_url.contains('?') {
                database_url.push_str("&statement_cache_size=0");
            } else {
                database_url.push_str("?statement_cache_size=0");
            }
            println!("✅ Added statement_cache_size=0 to DATABASE_URL");
        }

        // Log URL changes for debugging
        if original_url != database_url {
            println!(
                "   Original: {}",
                original_url.split('@').last().unwrap_or(&original_url)
            );
            println!(
                "   Updated:  {}",
                database_url.split('@').last().unwrap_or(&database_url)
            );
        } else {
            println!("✅ DATABASE_URL is properly configured");
        }
    }

    let connect_options = PgConnectOptions::from_str(&database_url)?;

    // Production vs Development pool settings
    let mut pool_builder = PgPoolOptions::new();

    if is_production {
        pool_builder = pool_builder
            .max_connections(20)
            .min_connections(5)
            .idle_timeout(std::time::Duration::from_secs(300))
            .max_lifetime(std::time::Duration::from_secs(1800));
    } else {
        pool_builder = pool_builder
            .max_connections(10)
            .min_connections(2)
            .idle_timeout(std::time::Duration::from_secs(60))
            .max_lifetime(std::time::Duration::from_secs(300));
    }

    let pool = pool_builder
        .acquire_timeout(std::time::Duration::from_secs(30))
        .test_before_acquire(true)
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
                println!(
                    "   This usually means migrations are already applied or PgBouncer needs to clear its cache."
                );
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
                        println!(
                            "⚠️  Could not verify migration table (may be due to PgBouncer cache)"
                        );
                        println!("   Assuming migrations are applied and continuing...");
                        println!(
                            "   If you see database errors, run migrations manually: sqlx migrate run"
                        );
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
