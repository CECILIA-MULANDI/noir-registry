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
}

#[derive(Deserialize)]
struct PackageInfo {
    name: String,
    github_repository_url: String,
    latest_version: Option<String>,
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

/// Adds a dependency to Nargo.toml
fn add_dependency_to_nargo_toml(
    manifest_path: &Path,
    package_name: &str,
    github_url: &str,
) -> Result<()> {
    // Read the file
    let content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;

    // Parse TOML using toml_edit for better formatting control
    let mut doc = content
        .parse::<DocumentMut>()
        .context("Failed to parse Nargo.toml")?;

    // Get or create [dependencies] section
    let deps = doc
        .entry("dependencies")
        .or_insert_with(|| Item::Table(Table::new()))
        .as_table_mut()
        .context("Failed to access dependencies section")?;

    // Check if dependency already exists
    if deps.contains_key(package_name) {
        anyhow::bail!("Dependency '{}' already exists in Nargo.toml", package_name);
    }

    // Add the dependency as an inline table
    // Format: package_name = { git = "github_url" }
    let mut dep_table = InlineTable::new();
    dep_table.insert("git", toml_edit::Value::from(github_url));

    deps.insert(
        package_name,
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
    if let Some(version) = package_info.latest_version {
        eprintln!("   Latest version: {}", version);
    }

    // Add to Nargo.toml
    match add_dependency_to_nargo_toml(
        &manifest_path,
        &args.package_name,
        &package_info.github_repository_url,
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
        }
        Err(e) => {
            eprintln!("❌ Failed to add dependency: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
