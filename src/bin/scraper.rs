use anyhow::Result;
use noir_registry::db;
use regex::Regex;
use serde::Deserialize;

/// This should contain the structure of the package we are scraping
#[derive(Debug, Clone)]
struct Package {
    name: String,
    github_url: String,
    description: String,
}
/// GitHub API response for repository info
#[derive(Debug, Deserialize)]
struct GitHubRepo {
    owner: GitHubOwner,
    stargazers_count: i32,
    license: Option<GitHubLicense>,
    homepage: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubOwner {
    login: String,
    avatar_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubLicense {
    spdx_id: String,
}
/// Enriched package with GitHub metadata
#[derive(Debug, Clone)]
struct EnrichedPackage {
    name: String,
    description: String,
    github_url: String,
    owner_username: String,
    owner_avatar: String,
    stars: i32,
    license: Option<String>,
    homepage: Option<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting the Noir package scraper...");
    // Load all env variables
    dotenvy::dotenv().ok();
    let github_token = std::env::var("GITHUB_TOKEN").ok();
    if github_token.is_some() {
        println!("🔑 Using GitHub authentication");
    } else {
        println!("⚠️  No GITHUB_TOKEN found - rate limited to 60 requests/hour");
    }

    // Connect to db
    println!("Connecting to database!");
    let pool = db::create_pool().await?;
    println!("✅ Connected to the database");

    // Fetch the awesome-noir README
    println!("Fetching awesome-noir README...");
    let readme_url = "https://raw.githubusercontent.com/noir-lang/awesome-noir/main/README.md";
    let readme_content = fetch_readme(readme_url).await?;
    println!("✅ Fetched README ({} bytes)", readme_content.len());
    // Parse the markdown to find libraries
    println!("Parsing packages for the README....");
    let packages = parse_packages(&readme_content)?;
    println!("✅ Found {} packages", packages.len());

    // Create HTTP client for GitHub API calls
    let client = reqwest::Client::new();
    println!("\n📡 Fetching GitHub metadata...");
    let mut enriched_packages = Vec::new();

    for (i, pkg) in packages.iter().enumerate() {
        print!("  [{}/{}] Fetching {}... ", i + 1, packages.len(), pkg.name);

        match enrich_package(&client, pkg, github_token.as_deref()).await {
            Ok(enriched) => {
                println!("✅ ({} stars)", enriched.stars);
                enriched_packages.push(enriched);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }

        // Be nice to GitHub API - add small delay
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    println!("\n✅ Enriched {} packages", enriched_packages.len());
    // Print sample enriched packages
    println!("\n📦 Sample enriched packages:");
    for pkg in enriched_packages.iter().take(3) {
        println!(
            "  • {} by @{} ({} ⭐)",
            pkg.name, pkg.owner_username, pkg.stars
        );
    }

    // Insert to the db
    println!("\n💾 Inserting packages into database...");
    let mut inserted_count = 0;
    let mut failed_count = 0;

    for pkg in enriched_packages.iter() {
        match insert_package(&pool, pkg).await {
            Ok(_) => {
                inserted_count += 1;
                print!(".");
            }
            Err(e) => {
                failed_count += 1;
                eprintln!("\n❌ Failed to insert {}: {}", pkg.name, e);
            }
        }
    }

    println!("\n✅ Inserted {} packages into database", inserted_count);
    if failed_count > 0 {
        println!("⚠️  {} packages failed to insert", failed_count);
    }

    //close connection
    pool.close().await;
    println!("✅ Scraping complete!");

    Ok(())
}

/// This function should be fetching the raw readme content from github
async fn fetch_readme(url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "noir-registry-scraper")
        .send()
        .await?;
    let content = response.text().await?;
    Ok(content)
}

/// Parses the README to extract package information
fn parse_packages(readme: &str) -> Result<Vec<Package>> {
    let mut packages = Vec::new();
    // Regex pattern to match: - [Name](url) - description
    // Pattern explanation:
    // - \[([^\]]+)\]  -> matches [Name] and captures "Name"
    // - \(([^)]+)\)   -> matches (url) and captures "url"
    // - \s*-\s*(.+)   -> matches " - description" and captures "description"
    let re = Regex::new(r"-\s*\[([^\]]+)\]\(([^)]+)\)\s*-\s*(.+)")?;
    for line in readme.lines() {
        if let Some(caps) = re.captures(line) {
            let name = caps
                .get(1)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            let url = caps
                .get(2)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            let description = caps
                .get(3)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();

            // Only include if it's a GitHub URL
            if url.contains("github.com") {
                packages.push(Package {
                    name,
                    github_url: url,
                    description,
                });
            }
        }
    }

    Ok(packages)
}

/// Extracts owner and repo name from github url
fn parse_github_url(url: &str) -> Option<(String, String)> {
    // This is the URL Pattern: https://github.com/owner/repo
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() >= 5 {
        let owner = parts[3].to_string();
        let repo = parts[4].to_string();
        return Some((owner, repo));
    }
    None
}
/// Fetches repository metadata from GitHub API
async fn fetch_github_metadata(
    client: &reqwest::Client,
    github_url: &str,
    token: Option<&str>,
) -> Result<GitHubRepo> {
    let (owner, repo) = parse_github_url(github_url)
        .ok_or_else(|| anyhow::anyhow!("Invalid GitHub URL: {}", github_url))?;

    let api_url = format!("https://api.github.com/repos/{}/{}", owner, repo);

    let mut request = client
        .get(&api_url)
        .header("User-Agent", "noir-registry-scraper")
        .header("Accept", "application/vnd.github.v3+json");

    // Add authentication if token is provided
    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        anyhow::bail!("GitHub API error: {}", response.status());
    }

    let repo_data: GitHubRepo = response.json().await?;
    Ok(repo_data)
}

/// Enriches a package with GitHub metadata
async fn enrich_package(
    client: &reqwest::Client,
    pkg: &Package,
    token: Option<&str>,
) -> Result<EnrichedPackage> {
    let github_data = fetch_github_metadata(client, &pkg.github_url, token).await?;

    Ok(EnrichedPackage {
        name: pkg.name.clone(),
        description: pkg.description.clone(),
        github_url: pkg.github_url.clone(),
        owner_username: github_data.owner.login,
        owner_avatar: github_data.owner.avatar_url,
        stars: github_data.stargazers_count,
        license: github_data.license.map(|l| l.spdx_id),
        homepage: github_data.homepage,
    })
}
/// Inserts an enriched package into the database
async fn insert_package(pool: &sqlx::PgPool, pkg: &EnrichedPackage) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO packages (
            name,
            description,
            github_repository_url,
            homepage,
            license,
            owner_github_username,
            owner_avatar_url,
            github_stars,
            total_downloads
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (name) DO UPDATE SET
            description = EXCLUDED.description,
            github_repository_url = EXCLUDED.github_repository_url,
            homepage = EXCLUDED.homepage,
            license = EXCLUDED.license,
            owner_github_username = EXCLUDED.owner_github_username,
            owner_avatar_url = EXCLUDED.owner_avatar_url,
            github_stars = EXCLUDED.github_stars,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(&pkg.name)
    .bind(&pkg.description)
    .bind(&pkg.github_url)
    .bind(&pkg.homepage)
    .bind(&pkg.license)
    .bind(&pkg.owner_username)
    .bind(&pkg.owner_avatar)
    .bind(pkg.stars)
    .bind(0i32) // total_downloads starts at 0
    .persistent(false) // Disable prepared statement caching to avoid conflicts
    .execute(pool)
    .await?;

    Ok(())
}
