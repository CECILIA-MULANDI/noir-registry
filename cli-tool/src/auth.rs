use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GitHubAuthRequest {
    pub github_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAuthResponse {
    pub success: bool,
    pub api_key: Option<String>,
    pub message: String,
    #[allow(dead_code)]
    pub github_username: Option<String>,
}

/// Authenticates with GitHub and returns API key
pub async fn authenticate_github(registry_url: &str, github_token: &str) -> Result<String> {
    let client = Client::new();
    let auth_url = format!("{}/auth/github", registry_url.trim_end_matches('/'));

    let response = client
        .post(&auth_url)
        .json(&GitHubAuthRequest {
            github_token: github_token.to_string(),
        })
        .send()
        .await
        .context("Failed to connect to registry")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!("Authentication failed: {}", error_text);
    }

    let auth_response: GitHubAuthResponse = response
        .json()
        .await
        .context("Failed to parse authentication response")?;

    if !auth_response.success {
        anyhow::bail!("Authentication failed: {}", auth_response.message);
    }

    auth_response
        .api_key
        .context("No API key received from authentication")
}
