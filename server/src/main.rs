use noir_registry_server::{db, rest_apis};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize database connection and run migrations
    let pool = db::init_db().await?;

    // Create the API router
    let app = rest_apis::create_router(pool);

    // Start the server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Server starting on http://{}", addr);
    println!("📡 Available endpoints:");
    println!("   GET /health - Health check");
    println!("   GET /api/packages - List all packages");
    println!("   GET /api/packages/:name - Get package by name");
    println!("   GET /api/search?q=query - Search packages");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("✅ Server running!");
    axum::serve(listener, app).await?;

    Ok(())
}

