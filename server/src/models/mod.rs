use serde::{Deserialize, Serialize};

/// A package category (e.g. Cryptography, Math)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

/// This should contain the structure of the package we are scraping
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub github_url: String,
    pub description: String,
}
/// This is the structure of the package we expect from an API response
#[derive(Debug, Clone, Serialize)]
pub struct PackageResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub github_repository_url: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub owner_github_username: String,
    pub owner_avatar_url: Option<String>,
    pub total_downloads: i32,
    pub github_stars: i32,
    pub latest_version: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub keywords: Vec<String>,
}
/// GitHub API response for repository info
#[derive(Debug, Deserialize)]
pub struct GitHubRepo {
    pub owner: GitHubOwner,
    pub stargazers_count: i32,
    pub license: Option<GitHubLicense>,
    pub homepage: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubOwner {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubLicense {
    pub spdx_id: String,
}
/// Enriched package with GitHub metadata
#[derive(Debug, Clone)]
pub struct EnrichedPackage {
    pub name: String,
    pub description: String,
    pub github_url: String,
    pub owner_username: String,
    pub owner_avatar: String,
    pub stars: i32,
    pub license: Option<String>,
    pub homepage: Option<String>,
}
