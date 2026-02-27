use anyhow::{Context, Result};
use clap::Parser;
use nargo_add::{nargo_toml, utils};
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml_edit::{DocumentMut, InlineTable, Item, Table};

#[derive(Parser)]
#[command(name = "nargo-add")]
#[command(about = "Add a package dependency from the Noir registry (use: nargo add <package>)")]
#[command(version)]
struct Args {
    /// Package name to add (e.g., rocq-of-noir)
    package_name: String,

    /// Registry API URL (optional, defaults to NOIR_REGISTRY_URL env var or http://localhost:8080/api)
    #[arg(long)]
    registry: Option<String>,

    /// Path to Nargo.toml (optional, will search from current directory)
    #[arg(long)]
    manifest_path: Option<std::path::PathBuf>,

    /// Skip running `nargo check` after adding the dependency
    #[arg(long)]
    no_fetch: bool,
}

#[derive(Deserialize)]
struct PackageInfo {
    name: String,
    github_repository_url: String,
    latest_version: Option<String>,
}

#[derive(Deserialize)]
struct GitHubTag {
    name: String,
}

/// Extracts the "{owner}/{repo}" slug from a GitHub URL.
/// Handles both https://github.com/owner/repo and https://github.com/owner/repo/tree/...
fn github_slug_from_url(url: &str) -> Option<String> {
    let url = url.trim_end_matches('/');
    let stripped = url.strip_prefix("https://github.com/")?;
    // Take only the first two path segments (owner/repo)
    let mut parts = stripped.splitn(3, '/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    Some(format!("{}/{}", owner, repo))
}

/// Fetches the latest tag name from the GitHub API for a given repo URL.
/// Returns None if the repo has no tags or the request fails (non-fatal).
async fn fetch_latest_github_tag(client: &Client, github_url: &str) -> Option<String> {
    let slug = github_slug_from_url(github_url)?;
    let api_url = format!("https://api.github.com/repos/{}/tags", slug);

    let response = client
        .get(&api_url)
        .header("User-Agent", "nargo-add")
        .header("Accept", "application/vnd.github+json")
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let tags: Vec<GitHubTag> = response.json().await.ok()?;
    tags.into_iter().next().map(|t| t.name)
}

/// Fetches package information from the registry with retry logic
async fn fetch_package_info(registry_url: &str, package_name: &str) -> Result<PackageInfo> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")?;

    let url = format!(
        "{}/packages/{}",
        registry_url.trim_end_matches('/'),
        package_name
    );

    // Retry logic: 3 attempts with exponential backoff
    let mut last_error: Option<anyhow::Error> = None;
    for attempt in 0..3 {
        let response = match client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                let err = anyhow::anyhow!("Network error: {}", e);
                last_error = Some(err);
                if attempt < 2 {
                    let delay = std::time::Duration::from_millis(100 * (1 << attempt));
                    tokio::time::sleep(delay).await;
                    continue;
                }
                return Err(last_error
                    .unwrap()
                    .context(format!("Failed to connect to registry at {}", url)));
            }
        };

        match response.status() {
            status if status.is_success() => match response.json::<PackageInfo>().await {
                Ok(package) => return Ok(package),
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse package response from registry: {}. \
                            The registry may be returning an unexpected format.",
                        e
                    ));
                }
            },
            status if status == 404 => {
                return Err(anyhow::anyhow!(
                    "Package '{}' not found in registry.\n\
                    Registry URL: {}\n\
                    Tip: Check the package name and ensure the registry is up to date.",
                    package_name,
                    registry_url
                ));
            }
            status if status == 503 || status == 502 => {
                last_error = Some(anyhow::anyhow!("Registry server error: {}", status));
                if attempt < 2 {
                    let delay = std::time::Duration::from_millis(500 * (1 << attempt));
                    eprintln!(
                        "⚠️  Registry temporarily unavailable, retrying in {:.1}s...",
                        delay.as_secs_f64()
                    );
                    tokio::time::sleep(delay).await;
                    continue;
                } else {
                    return Err(last_error
                        .unwrap()
                        .context("Registry server is unavailable"));
                }
            }
            status => {
                let error_text = response.text().await.unwrap_or_default();
                return Err(anyhow::anyhow!(
                    "Registry returned error {}: {}\n\
                    Registry URL: {}",
                    status,
                    error_text,
                    registry_url
                ));
            }
        }
    }

    Err(last_error
        .unwrap_or_else(|| anyhow::anyhow!("Failed to fetch package after 3 attempts"))
        .context("Registry request failed"))
}

/// Runs `nargo check` in the project directory to fetch and validate the new dependency.
/// Returns Ok(true) if nargo is installed and check passed, Ok(false) if nargo isn't found.
fn run_nargo_fetch(manifest_path: &Path) -> Result<bool> {
    use std::process::Command;

    // Run nargo check from the directory containing Nargo.toml
    let project_dir = manifest_path
        .parent()
        .context("Could not determine project directory from manifest path")?;

    let output = match Command::new("nargo")
        .arg("check")
        .current_dir(project_dir)
        .output()
    {
        Ok(out) => out,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // nargo not installed — not a fatal error, just warn
            return Ok(false);
        }
        Err(e) => return Err(anyhow::anyhow!("Failed to run nargo: {}", e)),
    };

    if output.status.success() {
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!(
            "nargo check failed after adding dependency:\n{}",
            stderr.trim()
        ))
    }
}

/// Nargo requires dependency keys to use underscores, not hyphens.
fn sanitize_dep_key(name: &str) -> String {
    name.replace('-', "_")
}

/// Adds a dependency to Nargo.toml.
/// `tag` is required by nargo ≥1.0.0-beta.16 for git dependencies.
fn add_dependency_to_nargo_toml(
    manifest_path: &Path,
    package_name: &str,
    github_url: &str,
    tag: Option<&str>,
) -> Result<()> {
    // Read the file
    let content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;

    // Parse TOML using toml_edit for better formatting control
    let mut doc = content
        .parse::<DocumentMut>()
        .context("Failed to parse Nargo.toml")?;

    // Nargo requires underscores in dependency keys (hyphens are invalid)
    let dep_key = sanitize_dep_key(package_name);

    // Get or create [dependencies] section
    let deps = doc
        .entry("dependencies")
        .or_insert_with(|| Item::Table(Table::new()))
        .as_table_mut()
        .context("Failed to access dependencies section")?;

    // Check if dependency already exists (check both hyphenated and underscored forms)
    if deps.contains_key(&dep_key) || deps.contains_key(package_name) {
        anyhow::bail!("Dependency '{}' already exists in Nargo.toml", package_name);
    }

    // Build the inline table: { git = "...", tag = "..." }
    // nargo ≥1.0.0-beta.16 requires `tag` for git deps.
    let mut dep_table = InlineTable::new();
    dep_table.insert("git", toml_edit::Value::from(github_url));
    if let Some(t) = tag {
        dep_table.insert("tag", toml_edit::Value::from(t));
    }

    deps.insert(
        &dep_key,
        Item::Value(toml_edit::Value::InlineTable(dep_table)),
    );

    // Write back
    fs::write(manifest_path, doc.to_string())
        .with_context(|| format!("Failed to write {}", manifest_path.display()))?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Get registry URL
    let registry_url = utils::get_registry_url(args.registry);

    // Find Nargo.toml
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let manifest_path = match args.manifest_path {
        Some(path) => {
            if !path.exists() {
                anyhow::bail!("Nargo.toml not found at: {}", path.display());
            }
            path
        }
        None => nargo_toml::find_nargo_toml(&current_dir)?,
    };

    eprintln!(
        "📦 Fetching package '{}' from registry...",
        args.package_name
    );
    eprintln!("   Registry: {}", registry_url);

    // Fetch package info
    let package_info = match fetch_package_info(&registry_url, &args.package_name).await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            eprintln!("\n💡 Troubleshooting:");
            eprintln!("   - Check that the registry server is running");
            eprintln!("   - Verify the package name is correct");
            eprintln!(
                "   - Try: curl {}/packages/{}",
                registry_url, args.package_name
            );
            return Err(e);
        }
    };

    eprintln!("✅ Found package: {}", package_info.name);
    eprintln!("   Repository: {}", package_info.github_repository_url);

    // Resolve the version to use: registry value → GitHub tag → none
    let resolved_version: Option<String> = if package_info.latest_version.is_some() {
        let v = package_info.latest_version.clone();
        eprintln!("   Latest version: {}", v.as_deref().unwrap());
        v
    } else {
        eprintln!("   Checking GitHub for latest tag...");
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap_or_default();
        match fetch_latest_github_tag(&client, &package_info.github_repository_url).await {
            Some(tag) => {
                eprintln!("   Latest tag: {} (from GitHub)", tag);
                Some(tag)
            }
            None => {
                eprintln!("   ⚠️  No version tag found — dependency will be added without a tag.");
                eprintln!("      Add a `tag` manually in Nargo.toml once the author publishes a release.");
                None
            }
        }
    };

    // Add to Nargo.toml
    match add_dependency_to_nargo_toml(
        &manifest_path,
        &args.package_name,
        &package_info.github_repository_url,
        resolved_version.as_deref(),
    ) {
        Ok(_) => {
            eprintln!(
                "✅ Added '{}' to {}",
                args.package_name,
                manifest_path.display()
            );

            // Validate the TOML was written correctly
            if let Err(e) = nargo_toml::validate_nargo_toml(&manifest_path) {
                eprintln!("⚠️  Warning: Could not validate Nargo.toml: {}", e);
                eprintln!("   Please check the file manually");
            }

            // Record the download — fire-and-forget, non-fatal
            let download_url = format!(
                "{}/packages/{}/download",
                registry_url.trim_end_matches('/'),
                args.package_name
            );
            let ping_client = Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default();
            let _ = ping_client.post(&download_url).send().await;
        }
        Err(e) => {
            eprintln!("❌ Failed to add dependency: {}", e);
            return Err(e);
        }
    }

    // Fetch and validate the dependency via `nargo check`
    // Skip if no tag is available — nargo ≥1.0.0-beta.16 requires `tag` for git deps,
    // so `nargo check` would fail anyway without one.
    if !args.no_fetch && resolved_version.is_some() {
        eprintln!("📥 Fetching dependency with `nargo check`...");
        match run_nargo_fetch(&manifest_path) {
            Ok(true) => {
                eprintln!("✅ Dependency fetched and validated successfully!");
            }
            Ok(false) => {
                eprintln!("⚠️  nargo not found in PATH — skipping fetch.");
                eprintln!(
                    "   Run `nargo check` manually to pull the dependency, or install nargo first."
                );
            }
            Err(e) => {
                eprintln!("⚠️  nargo check failed: {}", e);
                eprintln!("   The dependency was added to Nargo.toml but could not be fetched.");
                eprintln!("   This may be caused by other unresolved dependencies in your project.");
                eprintln!("   Run `nargo check` manually to see the full error, or");
                eprintln!("   run `nargo remove {}` to undo.", args.package_name);
            }
        }
    }

    Ok(())
}
