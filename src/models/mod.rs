use serde::Deserialize;

/// This should contain the structure of the package we are scraping
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub github_url: String,
    pub description: String,
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