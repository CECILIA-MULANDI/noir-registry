use axum::body::Body;
use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Json, Response},
    routing::get,
};

use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::PackageResponse;
use crate::package_storage;

use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, Any, CorsLayer};

/// This is the application state that we should share across all handlers

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
}
/// Query parameters for search endpoint
#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

/// Creates the API router with all routes

pub fn create_router(db: PgPool) -> Router {
    let state = Arc::new(AppState { db });

    // Production-safe CORS configuration
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "*".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    let cors = if allowed_origins.contains(&"*".to_string()) {
        // Development: allow all origins
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        // Production: specific origins only
        let origins: Vec<_> = allowed_origins.iter().map(|s| s.parse().unwrap()).collect();

        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods(AllowMethods::list([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::OPTIONS,
            ]))
            .allow_headers(AllowHeaders::list([axum::http::HeaderName::from_static(
                "content-type",
            )]))
    };

    Router::new()
        .route("/api/packages", get(list_packages))
        .route("/api/packages/:name", get(get_package))
        .route("/api/search", get(search))
        .route("/health", get(health_check))
        .layer(cors)
        .with_state(state)
}
/// A GET endpoint (/api/packages)to list all packages
async fn list_packages(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PackageResponse>>, Response> {
    match package_storage::get_all_packages(&state.db).await {
        Ok(packages) => Ok(Json(packages)),
        Err(e) => {
            let error_msg = e.to_string();
            eprintln!("Error fetching packages: {}", error_msg);

            // Provide helpful error message for prepared statement issues
            if error_msg.contains("prepared statement") {
                eprintln!("⚠️  PgBouncer prepared statement error detected!");
                eprintln!("   Solution: Add ?statement_cache_size=0 to your DATABASE_URL");
                eprintln!("   Or use direct connection (port 5432) instead of pooler (port 6543)");
            }

            // Return error with message in response body
            let response = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(format!(r#"{{"error": "{}"}}"#, error_msg)))
                .unwrap();
            Err(response)
        }
    }
}

/// A GET (api/packages/:name) endpoint to get a single package by name
async fn get_package(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<PackageResponse>, StatusCode> {
    match package_storage::get_package_by_name(&state.db, &name).await {
        Ok(Some(package)) => Ok(Json(package)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error fetching package '{}': {}", name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// A GET (/api/search?q=query) endpoint to search packages
async fn search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<PackageResponse>>, StatusCode> {
    match package_storage::search_packages(&state.db, &params.q).await {
        Ok(packages) => Ok(Json(packages)),
        Err(e) => {
            eprintln!("Error searching packages with query '{}': {}", params.q, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET (/health) endpoint to check health 
async fn health_check(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    // Check database connection
    match sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
    {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "healthy",
            "database": "connected",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))),
        Err(e) => {
            eprintln!("Health check failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

