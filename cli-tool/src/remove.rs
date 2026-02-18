use anyhow::{Context, Result};
use clap::Parser;
use nargo_add::nargo_toml;
use std::fs;
use std::path::Path;
use toml_edit::DocumentMut;

#[derive(Parser)]
#[command(name = "nargo-remove")]
#[command(about = "Remove a package dependency from Nargo.toml (use: nargo remove <package>)")]
#[command(version)]
struct Args {
    /// Package name(s) to remove
    #[arg(required = true)]
    package_names: Vec<String>,

    /// Path to Nargo.toml (optional, will search from current directory)
    #[arg(long)]
    manifest_path: Option<std::path::PathBuf>,
}

/// Removes a dependency from Nargo.toml.
/// Returns Ok(true) if the dependency was found and removed, Ok(false) if it wasn't present.
fn remove_dependency_from_nargo_toml(
    manifest_path: &Path,
    package_name: &str,
) -> Result<bool> {
    let content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;

    let mut doc = content
        .parse::<DocumentMut>()
        .context("Failed to parse Nargo.toml")?;

    // Get the [dependencies] table
    let deps = match doc.get_mut("dependencies").and_then(|d| d.as_table_mut()) {
        Some(deps) => deps,
        None => {
            // No [dependencies] section at all
            return Ok(false);
        }
    };

    // Check if the dependency exists
    if !deps.contains_key(package_name) {
        return Ok(false);
    }

    // Remove the dependency
    deps.remove(package_name);

    // Write back
    fs::write(manifest_path, doc.to_string())
        .with_context(|| format!("Failed to write {}", manifest_path.display()))?;

    Ok(true)
}

fn main() -> Result<()> {
    let args = Args::parse();

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

    let mut removed = Vec::new();
    let mut not_found = Vec::new();
    let mut errors = Vec::new();

    for package_name in &args.package_names {
        match remove_dependency_from_nargo_toml(&manifest_path, package_name) {
            Ok(true) => {
                eprintln!("✅ Removed '{}' from {}", package_name, manifest_path.display());
                removed.push(package_name.as_str());
            }
            Ok(false) => {
                eprintln!(
                    "⚠️  Dependency '{}' not found in {}",
                    package_name,
                    manifest_path.display()
                );
                not_found.push(package_name.as_str());
            }
            Err(e) => {
                eprintln!("❌ Failed to remove '{}': {}", package_name, e);
                errors.push(package_name.as_str());
            }
        }
    }

    // Validate the TOML is still well-formed after all removals
    if !removed.is_empty() {
        if let Err(e) = nargo_toml::validate_nargo_toml(&manifest_path) {
            eprintln!("⚠️  Warning: Could not validate Nargo.toml after removal: {}", e);
            eprintln!("   Please check the file manually");
        }
    }

    // Print summary when operating on multiple packages
    if args.package_names.len() > 1 {
        eprintln!();
        eprintln!("Summary: {} removed, {} not found, {} errors",
            removed.len(), not_found.len(), errors.len());
    }

    if !errors.is_empty() {
        anyhow::bail!("Some packages could not be removed");
    }

    if !not_found.is_empty() && removed.is_empty() {
        anyhow::bail!(
            "No matching dependencies found in {}",
            manifest_path.display()
        );
    }

    Ok(())
}
