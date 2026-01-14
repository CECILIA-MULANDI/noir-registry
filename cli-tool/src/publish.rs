use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::DocumentMut;
#[derive(Parser)]
#[command(name = "nargo-publish")]
#[command(about = "Publish a package to the Noir registry(use: nargo publish)")]
#[command(version)]
struct Args {
    #[arg(long)]
    registry: Option<String>,
    #[arg(long)]
    repo: Option<String>,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    package_version: Option<String>,
    #[arg(long)]
    license: Option<String>,
    #[arg(long)]
    homepage: Option<String>,
    #[arg(long)]
    github_token: Option<String>,
    #[arg(long)]
    manifest_path: Option<PathBuf>,
}

#[derive(Deserialize)]
struct PublishResponse {
    success: bool,
    message: String,
    #[allow(dead_code)]
    package_id: Option<i32>,
}

#[derive(Serialize)]
struct PublishRequest {
    name: String,
    description: Option<String>,
    github_repository_url: String,
    version: Option<String>,
    license: Option<String>,
    homepage: Option<String>,
}

#[derive(Deserialize)]
struct GitHubAuthResponse {
    success: bool,
    api_key: Option<String>,
    message: String,
    #[allow(dead_code)]
    github_username: Option<String>,
}

#[derive(Serialize)]
struct GitHubAuthRequest {
    github_token: String,
}
/// Get the registry URL from args, env var, or default
fn get_registry_url(args_registry: Option<String>) -> String {
    args_registry
        .or_else(|| std::env::var("NOIR_REGISTRY_URL").ok())
        .unwrap_or_else(|| "http://109.205.177.65/api".to_string())
}
/// Finds Nargo.toml
fn find_nargo_toml(start_dir: &Path) -> Result<PathBuf> {
    let mut current = start_dir.to_path_buf();
    loop {
        let manifest = current.join("Nargo.toml");
        if manifest.exists() {
            return Ok(manifest);
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => anyhow::bail!("Could not find Nargo.toml in current directory or parents"),
        }
    }
}
/// Reads package name from Nargo.toml
fn read_package_name(manifest_path: &Path) -> Result<String> {
    let content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;

    let doc = content
        .parse::<DocumentMut>()
        .context("Failed to parse Nargo.toml")?;

    let package_table = doc
        .get("package")
        .and_then(|p| p.as_table())
        .context("Nargo.toml does not contain [package] section")?;

    let name = package_table
        .get("name")
        .and_then(|n| n.as_str())
        .context("Package name not found in Nargo.toml")?;

    Ok(name.to_string())
}
/// Gets GitHub repository URL from git remote
fn get_git_remote_url() -> Result<String> {
    use std::process::Command;

    let output = Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .output()
        .context("Failed to run git command. Make sure git is installed.")?;

    if !output.status.success() {
        anyhow::bail!("Failed to get git remote URL. Is this a git repository?");
    }

    let url = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git remote URL")?
        .trim()
        .to_string();

    // Convert SSH URL to HTTPS URL if needed
    let url = if url.starts_with("git@github.com:") {
        url.replace("git@github.com:", "https://github.com/")
            .trim_end_matches(".git")
            .to_string()
    } else if url.ends_with(".git") {
        url.trim_end_matches(".git").to_string()
    } else {
        url
    };

    Ok(url)
}

/// Authenticates with GitHub and returns API key
async fn authenticate_github(registry_url: &str, github_token: &str) -> Result<String> {
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

/// Publishes a package to the registry
async fn publish_package(
    registry_url: &str,
    api_key: &str,
    request: &PublishRequest,
) -> Result<()> {
    let client = Client::new();
    let publish_url = format!("{}/packages/publish", registry_url.trim_end_matches('/'));

    let response = client
        .post(&publish_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(request)
        .send()
        .await
        .context("Failed to connect to registry")?;

    let status = response.status();
    let publish_response: PublishResponse = response
        .json()
        .await
        .context("Failed to parse publish response")?;

    if !publish_response.success {
        anyhow::bail!("Publish failed: {}", publish_response.message);
    }

    if !status.is_success() {
        anyhow::bail!(
            "Publish failed with status {}: {}",
            status,
            publish_response.message
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Get registry URL
    let registry_url = get_registry_url(args.registry);

    // Find Nargo.toml
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let manifest_path = match args.manifest_path {
        Some(path) => {
            if !path.exists() {
                anyhow::bail!("Nargo.toml not found at: {}", path.display());
            }
            path
        }
        None => find_nargo_toml(&current_dir)?,
    };

    eprintln!(
        "📦 Reading package information from {}",
        manifest_path.display()
    );

    // Read package name
    let package_name = read_package_name(&manifest_path)?;
    eprintln!("✅ Package name: {}", package_name);

    // Get GitHub repository URL
    let github_repo_url = if let Some(repo) = args.repo {
        repo
    } else {
        match get_git_remote_url() {
            Ok(url) => {
                eprintln!("✅ Detected repository: {}", url);
                url
            }
            Err(e) => {
                eprintln!("⚠️  Could not detect git remote: {}", e);
                eprintln!("   Please provide --repo <github-url> or run from a git repository");
                return Err(e);
            }
        }
    };

    // Get GitHub token (from arg or env var)
    let github_token = args.github_token
        .or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "GitHub token required. Provide --github-token <token> or set GITHUB_TOKEN env var.\n\
                Create a token at: https://github.com/settings/tokens (with 'repo' scope)"
            )
        })?;

    eprintln!("🔐 Authenticating with GitHub...");
    let api_key = authenticate_github(&registry_url, &github_token).await?;
    eprintln!("✅ Authentication successful");

    // Build publish request
    let publish_request = PublishRequest {
        name: package_name.clone(),
        description: args.description,
        github_repository_url: github_repo_url.clone(),
        version: args.package_version,
        license: args.license,
        homepage: args.homepage,
    };

    eprintln!("📤 Publishing package to registry...");
    eprintln!("   Registry: {}", registry_url);
    eprintln!("   Package: {}", publish_request.name);
    eprintln!("   Repository: {}", publish_request.github_repository_url);

    match publish_package(&registry_url, &api_key, &publish_request).await {
        Ok(_) => {
            eprintln!("✅ Package '{}' published successfully!", package_name);
            eprintln!(
                "   View at: {}/packages/{}",
                registry_url.replace("/api", ""),
                package_name
            );
        }
        Err(e) => {
            eprintln!("❌ Failed to publish package: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
