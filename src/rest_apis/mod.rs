use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::PackageResponse;
use crate::package_storage;

/// This is the application state that we should share across all handlers

#[derive(Debug, Clone)]
pub struct AppState{
    pub db:PgPool
}
/// Query parameters for search endpoint
#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

/// Creates the API router with all routes
pub fn create_router(db: PgPool) -> Router {
    let state = Arc::new(AppState { db });

    Router::new()
        .route("/api/packages", get(list_packages))
        .route("/api/packages/:name", get(get_package))
        .route("/api/search", get(search))
        .route("/health", get(health_check))
        .with_state(state)
}

/// A GET endpoint (/api/packages)to list all packages
async fn list_packages(State(state):State<Arc<AppState>>)->Result<Json<Vec<PackageResponse>>, StatusCode> {
    match package_storage::get_all_packages(&state.db).await {
        Ok(packages)=>Ok(Json(packages)),
        Err(e)=>{
            eprintln!("Error fetching packages: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
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
        },
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
        },
    }
}

/// GET (/health)endpoint to check health 
async fn health_check() -> &'static str {
    "OK"
}