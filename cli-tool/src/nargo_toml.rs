use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::DocumentMut;

/// Finds Nargo.toml by walking up from the current directory
pub fn find_nargo_toml(start_dir: &Path) -> Result<PathBuf> {
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
pub fn read_package_name(manifest_path: &Path) -> Result<String> {
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

/// Validates that the Nargo.toml file is valid TOML
pub fn validate_nargo_toml(manifest_path: &Path) -> Result<()> {
    let content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;

    content
        .parse::<DocumentMut>()
        .context("Nargo.toml is not valid TOML")?;

    Ok(())
}

/// Removes a dependency from Nargo.toml (used for rollback).
/// Returns Ok(true) if removed, Ok(false) if the dependency was not present.
pub fn remove_dependency(manifest_path: &Path, package_name: &str) -> Result<bool> {
    let content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read {}", manifest_path.display()))?;

    let mut doc = content
        .parse::<DocumentMut>()
        .context("Failed to parse Nargo.toml")?;

    let deps = match doc.get_mut("dependencies").and_then(|d| d.as_table_mut()) {
        Some(deps) => deps,
        None => return Ok(false),
    };

    if !deps.contains_key(package_name) {
        return Ok(false);
    }

    deps.remove(package_name);

    fs::write(manifest_path, doc.to_string())
        .with_context(|| format!("Failed to write {}", manifest_path.display()))?;

    Ok(true)
}
