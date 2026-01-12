use crate::auth;
use crate::models::PackageResponse;
use crate::package_storage;
use anyhow::Result;
use axum::body::Body;
use axum::{
    Router,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{Json, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
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
#[derive(Debug, Deserialize)]
pub struct PublishRequest {
    pub name: String,
    pub description: Option<String>,
    pub github_repository_url: String,
    pub version: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct PublishResponse {
    pub success: bool,
    pub message: String,
    pub package_id: Option<i32>,
}
/// Creates the API router with all routes

pub fn create_router(db: PgPool) -> Router {
    let state = Arc::new(AppState { db });

    // CORS configuration
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
        .route("/api/packages/publish", post(publish_package))
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
async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Check database connection
    match sqlx::query("SELECT 1").execute(&state.db).await {
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
/// POST /api/packages/publish
/// Requires: Authorization:Bearer <api_key> header
pub async fn publish_package(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<PublishRequest>,
) -> Result<Json<PublishResponse>, StatusCode> {
    // Extract API key from Authorization header
    let api_key = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| {
            eprintln!("Missing Authorization header");
            StatusCode::UNAUTHORIZED
        })?;

    // Validate API key and get user
    let user = auth::validate_api_key(&state.db, api_key)
        .await
        .map_err(|e| {
            eprintln!("Error validating API key: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            eprintln!("Invalid API key");
            StatusCode::UNAUTHORIZED
        })?;

    // Parse GitHub URL to get owner/repo
    let (owner, _repo) =
        parse_github_url(&payload.github_repository_url).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Verify GitHub repo ownership (simplified - I'll enhance this later)
    // For now, I'll trust the API key and assume user has access

    // Validate package name
    if !is_valid_package_name(&payload.name) {
        return Ok(Json(PublishResponse {
            success: false,
            message:
                "Invalid package name. Must be alphanumeric with hyphens/underscores, max 50 chars"
                    .to_string(),
            package_id: None,
        }));
    }

    // Insert or update package
    match insert_or_update_package(&state.db, &payload, user.id, &owner).await {
        Ok(package_id) => Ok(Json(PublishResponse {
            success: true,
            message: "Package published successfully".to_string(),
            package_id: Some(package_id),
        })),
        Err(e) => {
            eprintln!("Error publishing package: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
/// Validate package name (alphanumeric, hyphens, underscores)
fn is_valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 50
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Parse GitHub URL to extract owner and repo
fn parse_github_url(url: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() >= 5 && url.contains("github.com") {
        Ok((
            parts[3].to_string(),
            parts[4].trim_end_matches(".git").to_string(),
        ))
    } else {
        Err(anyhow::anyhow!("Invalid GitHub URL"))
    }
}

/// Insert or update package in database
async fn insert_or_update_package(
    pool: &PgPool,
    payload: &PublishRequest,
    user_id: i32,
    owner: &str,
) -> Result<i32> {
    let result = sqlx::query!(
        r#"
        INSERT INTO packages (
            name, description, github_repository_url, homepage, license,
            owner_github_username, published_by, source
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, 'user-published')
        ON CONFLICT (name) DO UPDATE SET
            description = EXCLUDED.description,
            github_repository_url = EXCLUDED.github_repository_url,
            homepage = EXCLUDED.homepage,
            license = EXCLUDED.license,
            updated_at = CURRENT_TIMESTAMP,
            published_by = EXCLUDED.published_by
        RETURNING id
        "#,
        payload.name,
        payload.description,
        payload.github_repository_url,
        payload.homepage,
        payload.license,
        owner,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.id)
}
