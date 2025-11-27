use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, InlineTable, Table};

#[derive(Parser)]
#[command(name = "nargo-add")]
#[command(about = "Add a package dependency from the Noir registry")]
struct Args {
    /// Package name to add (e.g., rocq-of-noir)
    package_name: String,

    /// Registry API URL (optional, defaults to http://localhost:8080/api)
    #[arg(long, default_value = "http://localhost:8080/api")]
    registry: String,

    /// Path to Nargo.toml (optional, will search from current directory)
    #[arg(long)]
    manifest_path: Option<PathBuf>,
}

#[derive(Deserialize)]
struct PackageInfo {
    name: String,
    github_repository_url: String,
    latest_version: Option<String>,
}

/// Finds Nargo.toml by walking up from the current directory
fn find_nargo_toml(start_dir: &Path) -> Result<PathBuf> {
    let mut current = start_dir.to_path_buf();

    loop {
        let manifest = current.join("Nargo.toml");
        if manifest.exists() {
            return Ok(manifest);
        }

        // Go up one directory
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => anyhow::bail!("Could not find Nargo.toml in current directory or parents"),
        }
    }
}

/// Fetches package information from the registry
async fn fetch_package_info(registry_url: &str, package_name: &str) -> Result<PackageInfo> {
    let client = Client::new();
    let url = format!("{}/packages/{}", registry_url.trim_end_matches('/'), package_name);

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch package from registry")?;

    if !response.status().is_success() {
        anyhow::bail!("Package '{}' not found in registry", package_name);
    }

    let package: PackageInfo = response
        .json()
        .await
        .context("Failed to parse package response")?;

    Ok(package)
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
        anyhow::bail!(
            "Dependency '{}' already exists in Nargo.toml",
            package_name
        );
    }

    // Add the dependency as an inline table
    // Format: package_name = { git = "github_url" }
    let mut dep_table = InlineTable::new();
    dep_table.insert("git", toml_edit::Value::from(github_url));
    
    deps.insert(package_name, Item::Value(toml_edit::Value::InlineTable(dep_table)));

    // Write back
    fs::write(manifest_path, doc.to_string())
        .with_context(|| format!("Failed to write {}", manifest_path.display()))?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Find Nargo.toml
    let current_dir = std::env::current_dir()?;
    let manifest_path = match args.manifest_path {
        Some(path) => path,
        None => find_nargo_toml(&current_dir)?,
    };

    println!("📦 Fetching package '{}' from registry...", args.package_name);

    // Fetch package info
    let package_info = fetch_package_info(&args.registry, &args.package_name).await?;

    println!("✅ Found package: {}", package_info.name);
    println!("   Repository: {}", package_info.github_repository_url);
    if let Some(version) = package_info.latest_version {
        println!("   Latest version: {}", version);
    }

    // Add to Nargo.toml
    add_dependency_to_nargo_toml(&manifest_path, &args.package_name, &package_info.github_repository_url)?;

    println!("✅ Added '{}' to {}", args.package_name, manifest_path.display());

    Ok(())
}

