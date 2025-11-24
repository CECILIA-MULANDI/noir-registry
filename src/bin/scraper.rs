use anyhow::Result;
use noir_registry::db;
use noir_registry::models::Package;
use noir_registry::github_metadata::enrich_package;
use noir_registry::package_storage::insert_package;
use regex::Regex;

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