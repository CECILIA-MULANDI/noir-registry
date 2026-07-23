use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use nargo_add::{config, utils};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "nargo-token")]
#[command(about = "Manage API tokens for the Noir registry (use: nargo token <command>)")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// Registry API URL (optional, defaults to NOIR_REGISTRY_URL env var)
    #[arg(long, global = true)]
    registry: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    /// List all tokens on your account
    List,
    /// Create a new named token. Raw value is printed exactly once.
    Create {
        /// Human-readable name for the token (e.g. "laptop", "ci")
        name: String,
        /// Also overwrite the stored token in ~/.config/noir-registry/config.toml
        #[arg(long)]
        save: bool,
    },
    /// Revoke a token by id
    Revoke {
        /// Numeric token id (see `nargo token list`)
        id: i32,
    },
}

#[derive(Debug, Deserialize)]
struct ApiToken {
    id: i32,
    name: String,
    token_prefix: String,
    created_at: String,
    last_used_at: Option<String>,
    revoked_at: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateTokenRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateTokenResponse {
    #[allow(dead_code)]
    token: ApiToken,
    raw: String,
    message: String,
}

fn load_api_key() -> Result<String> {
    let cfg = config::Config::load().context("Failed to load config")?;
    cfg.get_api_key()
        .map(|s| s.to_string())
        .context("Not logged in. Run 'nargo login' first, or set an API key via the CLI.")
}

async fn list(registry_url: &str, api_key: &str) -> Result<()> {
    let client = Client::new();
    let url = format!("{}/tokens", registry_url.trim_end_matches('/'));

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .context("Failed to connect to registry")?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("List tokens failed ({}): {}", status, body);
    }

    let tokens: Vec<ApiToken> = response
        .json()
        .await
        .context("Failed to parse tokens response")?;

    if tokens.is_empty() {
        println!("No tokens on this account.");
        return Ok(());
    }

    println!(
        "{:<5} {:<20} {:<12} {:<28} {:<28} {:<28}",
        "ID", "NAME", "PREFIX", "CREATED", "LAST USED", "REVOKED"
    );
    for t in tokens {
        println!(
            "{:<5} {:<20} {:<12} {:<28} {:<28} {:<28}",
            t.id,
            truncate(&t.name, 20),
            t.token_prefix,
            t.created_at,
            t.last_used_at.as_deref().unwrap_or("-"),
            t.revoked_at.as_deref().unwrap_or("-"),
        );
    }
    Ok(())
}

async fn create(registry_url: &str, api_key: &str, name: String, save: bool) -> Result<()> {
    let client = Client::new();
    let url = format!("{}/tokens", registry_url.trim_end_matches('/'));

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&CreateTokenRequest { name: name.clone() })
        .send()
        .await
        .context("Failed to connect to registry")?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Create token failed ({}): {}", status, body);
    }

    let created: CreateTokenResponse =
        response.json().await.context("Failed to parse create response")?;

    println!("Token '{}' created.", name);
    println!("{}", created.message);
    println!();
    println!("  {}", created.raw);
    println!();

    if save {
        let mut cfg = config::Config::load().context("Failed to load config")?;
        cfg.set_api_key(created.raw);
        cfg.save().context("Failed to save config")?;
        println!("Saved as the active token in your local config.");
    }

    Ok(())
}

async fn revoke(registry_url: &str, api_key: &str, id: i32) -> Result<()> {
    let client = Client::new();
    let url = format!("{}/tokens/{}", registry_url.trim_end_matches('/'), id);

    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .context("Failed to connect to registry")?;

    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Token {} revoked.", id);
            Ok(())
        }
        StatusCode::NOT_FOUND => {
            anyhow::bail!("Token {} not found (or not yours, or already revoked).", id)
        }
        other => {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Revoke failed ({}): {}", other, body)
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let registry_url = utils::get_registry_url(args.registry);
    let api_key = load_api_key()?;

    match args.command {
        Command::List => list(&registry_url, &api_key).await,
        Command::Create { name, save } => create(&registry_url, &api_key, name, save).await,
        Command::Revoke { id } => revoke(&registry_url, &api_key, id).await,
    }
}
