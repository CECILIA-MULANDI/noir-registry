use crate::auth;
use crate::models::{Category, PackageResponse};
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

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
}

/// Query parameters for /api/packages (optional keyword / category filter)
#[derive(Deserialize)]
pub struct ListPackagesQuery {
    pub keyword: Option<String>,
    pub category: Option<String>,
}

/// Query parameters for /api/search
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
    pub keywords: Option<Vec<String>>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublishResponse {
    pub success: bool,
    pub message: String,
    pub package_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAuthRequest {
    pub github_token: String,
}

#[derive(Debug, Serialize)]
pub struct GitHubAuthResponse {
    pub success: bool,
    pub api_key: Option<String>,
    pub message: String,
    pub github_username: Option<String>,
}

/// Creates the API router with all routes
pub fn create_router(db: PgPool) -> Router {
    let state = Arc::new(AppState { db });

    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "*".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    let cors = if allowed_origins.contains(&"*".to_string()) {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
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
        .route("/api/packages/:name/download", post(record_download))
        .route("/api/auth/github", post(github_auth))
        .route("/api/keywords", get(get_keywords))
        .route("/api/categories", get(get_categories))
        .layer(cors)
        .with_state(state)
}

/// GET /api/packages — list all packages, optionally filtered by keyword or category
async fn list_packages(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListPackagesQuery>,
) -> Result<Json<Vec<PackageResponse>>, Response> {
    let result = if let Some(keyword) = params.keyword {
        package_storage::get_packages_by_keyword(&state.db, &keyword).await
    } else if let Some(category) = params.category {
        package_storage::get_packages_by_category(&state.db, &category).await
    } else {
        package_storage::get_all_packages(&state.db).await
    };

    match result {
        Ok(packages) => Ok(Json(packages)),
        Err(e) => {
            let error_msg = e.to_string();
            eprintln!("Error fetching packages: {}", error_msg);

            if error_msg.contains("prepared statement") {
                eprintln!("⚠️  PgBouncer prepared statement error detected!");
                eprintln!("   Solution: Add ?statement_cache_size=0 to your DATABASE_URL");
                eprintln!("   Or use direct connection (port 5432) instead of pooler (port 6543)");
            }

            let response = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(format!(r#"{{"error": "{}"}}"#, error_msg)))
                .unwrap();
            Err(response)
        }
    }
}

/// GET /api/packages/:name — get a single package by name
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

/// GET /api/search?q=query — search by name, description, or keyword
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

/// GET /api/keywords — list all unique keywords
async fn get_keywords(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, StatusCode> {
    match package_storage::get_all_keywords(&state.db).await {
        Ok(keywords) => Ok(Json(keywords)),
        Err(e) => {
            eprintln!("Error fetching keywords: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/categories — list all package categories
async fn get_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    match package_storage::get_all_categories(&state.db).await {
        Ok(categories) => Ok(Json(categories)),
        Err(e) => {
            eprintln!("Error fetching categories: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// POST /api/packages/:name/download — increment download counter
async fn record_download(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> StatusCode {
    match package_storage::increment_downloads(&state.db, &name).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            eprintln!("Error recording download for '{}': {}", name, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// GET /health — health check
async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match sqlx::raw_sql("SELECT 1").execute(&state.db).await {
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

/// POST /api/auth/github — authenticate with GitHub token, return API key
pub async fn github_auth(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GitHubAuthRequest>,
) -> Result<Json<GitHubAuthResponse>, StatusCode> {
    match auth::get_or_create_user_from_github(&state.db, &payload.github_token).await {
        Ok(user) => {
            if let Some(api_key) = &user.api_key {
                Ok(Json(GitHubAuthResponse {
                    success: true,
                    api_key: Some(api_key.clone()),
                    message: "Authentication successful".to_string(),
                    github_username: Some(user.github_username.clone()),
                }))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(e) => {
            eprintln!("Error authenticating with Github: {}", e);
            Ok(Json(GitHubAuthResponse {
                success: false,
                api_key: None,
                message: format!("Failed to authenticate with GitHub: {}", e),
                github_username: None,
            }))
        }
    }
}

/// POST /api/packages/publish — publish a package (requires Bearer API key)
pub async fn publish_package(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<PublishRequest>,
) -> Result<Json<PublishResponse>, StatusCode> {
    let api_key = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| {
            eprintln!("Missing Authorization header");
            StatusCode::UNAUTHORIZED
        })?;

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

    let (owner, repo) =
        parse_github_url(&payload.github_repository_url).map_err(|_| StatusCode::BAD_REQUEST)?;

    match verify_github_ownership(&owner, &repo, &user.github_username).await {
        Ok(true) => {}
        Ok(false) => {
            return Ok(Json(PublishResponse {
                success: false,
                message: format!(
                    "You don't have permission to publish this package. \
                     The repository owner '{}' doesn't match your GitHub username '{}'",
                    owner, user.github_username
                ),
                package_id: None,
            }));
        }
        Err(e) => {
            eprintln!("Error verifying GitHub ownership: {}", e);
            return Ok(Json(PublishResponse {
                success: false,
                message: format!("Failed to verify repository ownership: {}", e),
                package_id: None,
            }));
        }
    }

    if !is_valid_package_name(&payload.name) {
        return Ok(Json(PublishResponse {
            success: false,
            message: "Invalid package name. Must be alphanumeric with hyphens/underscores, max 50 chars"
                .to_string(),
            package_id: None,
        }));
    }

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

/// Verify that a user owns a GitHub repository
async fn verify_github_ownership(
    owner: &str,
    repo: &str,
    user_github_username: &str,
) -> Result<bool> {
    let client = reqwest::Client::new();
    let api_url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    eprintln!(
        "🔍 Verifying ownership: repo={}/{}, user={}",
        owner, repo, user_github_username
    );
    let response = client
        .get(&api_url)
        .header("User-Agent", "noir-registry")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    if !response.status().is_success() {
        if response.status() == 404 {
            return Err(anyhow::anyhow!("Repository not found: {}/{}", owner, repo));
        }
        return Err(anyhow::anyhow!("GitHub API error: {}", response.status()));
    }

    let repo_data: serde_json::Value = response.json().await?;
    let repo_owner = repo_data
        .get("owner")
        .and_then(|o| o.get("login"))
        .and_then(|l| l.as_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to parse repository owner"))?;
    eprintln!(
        "🔍 Repo owner: '{}', User: '{}', Match: {}",
        repo_owner,
        user_github_username,
        repo_owner.eq_ignore_ascii_case(user_github_username)
    );

    Ok(repo_owner.eq_ignore_ascii_case(user_github_username))
}

fn is_valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 50
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

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

/// Insert or update package, then save keywords and category
async fn insert_or_update_package(
    pool: &PgPool,
    payload: &PublishRequest,
    user_id: i32,
    owner: &str,
) -> Result<i32> {
    use sqlx::Row;
    use crate::package_storage::escape_sql_string;

    fn sql_opt(opt: &Option<String>) -> String {
        match opt {
            None => "NULL".to_string(),
            Some(s) => format!("'{}'", escape_sql_string(s)),
        }
    }

    let sql = format!(
        r#"INSERT INTO packages (
            name, description, github_repository_url, homepage, license,
            owner_github_username, published_by, source
        ) VALUES ('{}', {}, '{}', {}, {}, '{}', {}, 'user-published')
        ON CONFLICT (name) DO UPDATE SET
            description = EXCLUDED.description,
            github_repository_url = EXCLUDED.github_repository_url,
            homepage = EXCLUDED.homepage,
            license = EXCLUDED.license,
            updated_at = CURRENT_TIMESTAMP,
            published_by = EXCLUDED.published_by
        RETURNING id"#,
        escape_sql_string(&payload.name),
        sql_opt(&payload.description),
        escape_sql_string(&payload.github_repository_url),
        sql_opt(&payload.homepage),
        sql_opt(&payload.license),
        escape_sql_string(owner),
        user_id,
    );
    let row = sqlx::raw_sql(&sql).fetch_one(pool).await?;

    let package_id: i32 = row.try_get("id")?;

    // Save keywords if provided
    if let Some(keywords) = &payload.keywords {
        if !keywords.is_empty() {
            package_storage::save_keywords(pool, package_id, keywords).await?;
        }
    }

    // Save category if provided
    if let Some(category_slug) = &payload.category {
        if !category_slug.is_empty() {
            if let Err(e) =
                package_storage::save_package_category(pool, package_id, category_slug).await
            {
                eprintln!("Warning: could not save category '{}': {}", category_slug, e);
            }
        }
    }

    Ok(package_id)
}
