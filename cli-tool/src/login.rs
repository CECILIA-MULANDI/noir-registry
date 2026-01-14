use anyhow::Result;
use clap::Parser;
use nargo_add::{auth, config, utils};

#[derive(Parser)]
#[command(name = "nargo-login")]
#[command(about = "Login to the Noir registry (use: nargo login)")]
#[command(version)]
struct Args {
    /// GitHub token for authentication (optional, can use env var GITHUB_TOKEN)
    #[arg(long)]
    github_token: Option<String>,

    /// Registry API URL (optional, defaults to NOIR_REGISTRY_URL env var)
    #[arg(long)]
    registry: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let registry_url = utils::get_registry_url(args.registry);

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
    let api_key = auth::authenticate_github(&registry_url, &github_token).await?;
    eprintln!("✅ Authentication successful");

    // Save API key to config
    let mut cfg = config::Config::load()?;
    cfg.set_api_key(api_key.clone());
    cfg.set_registry_url(registry_url.clone());
    cfg.save()?;

    eprintln!("✅ Credentials saved successfully!");
    eprintln!("   You can now use 'nargo publish' without authentication");

    Ok(())
}
