mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Initialize database connection and run migrations
    let _pool = db::init_db().await?;
    
    Ok(())
}
