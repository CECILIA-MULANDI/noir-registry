use crate::models::{EnrichedPackage, GitHubRepo, Package};
use anyhow::Result;
pub fn parse_github_url(url: &str) -> Option<(String, String)> {
    // This is the URL Pattern: https://github.com/owner/repo
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() >= 5 {
        let owner = parts[3].to_string();
        let repo = parts[4].to_string();
        return Some((owner, repo));
    }
    None
}
/// Fetches repository metadata from GitHub API
pub async fn fetch_github_metadata(
    client: &reqwest::Client,
    github_url: &str,
    token: Option<&str>,
) -> Result<GitHubRepo> {
    let (owner, repo) = parse_github_url(github_url)
        .ok_or_else(|| anyhow::anyhow!("Invalid GitHub URL: {}", github_url))?;

    let api_url = format!("https://api.github.com/repos/{}/{}", owner, repo);

    let mut request = client
        .get(&api_url)
        .header("User-Agent", "noir-registry-scraper")
        .header("Accept", "application/vnd.github.v3+json");

    // Add authentication if token is provided
    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        anyhow::bail!("GitHub API error: {}", response.status());
    }

    let repo_data: GitHubRepo = response.json().await?;
    Ok(repo_data)
}

/// Enriches a package with GitHub metadata
pub async fn enrich_package(
    client: &reqwest::Client,
    pkg: &Package,
    token: Option<&str>,
) -> Result<EnrichedPackage> {
    let github_data = fetch_github_metadata(client, &pkg.github_url, token).await?;

    Ok(EnrichedPackage {
        name: pkg.name.clone(),
        description: pkg.description.clone(),
        github_url: pkg.github_url.clone(),
        owner_username: github_data.owner.login,
        owner_avatar: github_data.owner.avatar_url,
        stars: github_data.stargazers_count,
        license: github_data.license.map(|l| l.spdx_id),
        homepage: github_data.homepage,
    })
}
