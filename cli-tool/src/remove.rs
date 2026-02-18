use anyhow::{Context, Result};
use clap::Parser;
use nargo_add::nargo_toml;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::DocumentMut;
use url::Url;

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

    /// Also delete cached source files from ~/nargo
    #[arg(long)]
    clean: bool,
}

/// Removes a dependency from Nargo.toml.
/// Returns Ok(Some(git_url)) if the dependency was found and removed, Ok(None) if it wasn't present.
fn remove_dependency_from_nargo_toml(
    manifest_path: &Path,
    package_name: &str,
) -> Result<Option<String>> {
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
            return Ok(None);
        }
    };

    // Check if the dependency exists and extract the git URL before removing
    let git_url = deps
        .get(package_name)
        .and_then(|item| {
            // Could be an inline table like { git = "url" } or a regular table
            if let Some(t) = item.as_inline_table() {
                t.get("git").and_then(|v| v.as_str()).map(|s| s.to_string())
            } else if let Some(t) = item.as_table() {
                t.get("git").and_then(|v| v.as_str()).map(|s| s.to_string())
            } else {
                None
            }
        });

    if !deps.contains_key(package_name) {
        return Ok(None);
    }

    // Remove the dependency
    deps.remove(package_name);

    // Write back
    fs::write(manifest_path, doc.to_string())
        .with_context(|| format!("Failed to write {}", manifest_path.display()))?;

    Ok(Some(git_url.unwrap_or_default()))
}

/// Derives the nargo cache directory for a git dependency URL.
/// Nargo caches git deps at ~/nargo/<domain>/<owner>/<repo>/
fn get_cache_dir_for_git_url(git_url: &str) -> Option<PathBuf> {
    let url = Url::parse(git_url).ok()?;
    let host = url.host_str()?;

    // Path segments: /<owner>/<repo> — strip leading slash and .git suffix
    let path = url.path().trim_start_matches('/').trim_end_matches(".git");
    if path.is_empty() {
        return None;
    }

    let home = dirs::home_dir()?;
    Some(home.join("nargo").join(host).join(path))
}

/// Deletes the cached source directory for a dependency.
fn clean_cached_source(git_url: &str) -> Result<bool> {
    if git_url.is_empty() {
        eprintln!("   ⚠️  No git URL found — cannot determine cache path");
        return Ok(false);
    }

    let cache_dir = match get_cache_dir_for_git_url(git_url) {
        Some(dir) => dir,
        None => {
            eprintln!("   ⚠️  Could not parse git URL '{}' — skipping cache cleanup", git_url);
            return Ok(false);
        }
    };

    if !cache_dir.exists() {
        eprintln!("   ℹ️  No cached files found at {}", cache_dir.display());
        return Ok(false);
    }

    fs::remove_dir_all(&cache_dir)
        .with_context(|| format!("Failed to delete cache at {}", cache_dir.display()))?;

    eprintln!("   🗑️  Deleted cached source: {}", cache_dir.display());
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
            Ok(Some(git_url)) => {
                eprintln!("✅ Removed '{}' from {}", package_name, manifest_path.display());
                if args.clean {
                    if let Err(e) = clean_cached_source(&git_url) {
                        eprintln!("   ⚠️  Failed to clean cache for '{}': {}", package_name, e);
                    }
                }
                removed.push(package_name.as_str());
            }
            Ok(None) => {
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
