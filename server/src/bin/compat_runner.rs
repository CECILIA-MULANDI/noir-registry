use anyhow::Result;
use noir_registry_server::db;
use noir_registry_server::package_storage::escape_sql_string;
use sqlx::Row;
use std::path::PathBuf;
use std::time::Duration;
use tokio::process::Command;

const CHECK_TIMEOUT: Duration = Duration::from_secs(90);

struct PackageInfo {
    id: i32,
    name: String,
    github_url: String,
}

enum CheckOutcome {
    Ok,
    Failed(String),
    Error(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    println!("Starting compat runner...");

    let nargo_version = detect_nargo_version().await?;
    println!("Nargo version detected: {}", nargo_version);

    let pool = db::create_pool().await?;
    println!("Connected to database.");

    let packages = fetch_target_packages(&pool).await?;
    println!("Selected {} packages for compat check.\n", packages.len());

    for (i, pkg) in packages.iter().enumerate() {
        println!("=== [{}/{}] {} ({}) ===", i + 1, packages.len(), pkg.name, pkg.github_url);
        let outcome = check_package(pkg).await;
        record_result(&pool, pkg, &nargo_version, &outcome).await?;
        print_outcome(&outcome);
        println!();
    }

    pool.close().await;
    println!("Done.");
    Ok(())
}

async fn detect_nargo_version() -> Result<String> {
    let output = Command::new("nargo")
        .arg("--version")
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("failed to invoke nargo (is it on PATH?): {}", e))?;

    if !output.status.success() {
        anyhow::bail!("nargo --version returned non-zero");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = stdout
        .lines()
        .find(|l| l.contains("nargo version"))
        .and_then(|l| l.split('=').nth(1))
        .map(|v| v.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    Ok(version)
}

async fn fetch_target_packages(pool: &sqlx::PgPool) -> Result<Vec<PackageInfo>> {
    // MVP: hand-picked known-standalone Noir libraries.
    // Ranking by stars surfaces apps that use Noir but are not libraries themselves.
    // Broaden to auto-detection later (probe for Nargo.toml during scrape).
    let query = r#"
        SELECT id, name, github_repository_url
        FROM packages
        WHERE name IN ('BigNum', 'Poseidon', 'ECDSA', 'Base64', 'BigCurve')
        ORDER BY name ASC
    "#;

    let rows = sqlx::raw_sql(query).fetch_all(pool).await?;
    let packages = rows
        .into_iter()
        .map(|r| {
            Ok(PackageInfo {
                id: r.try_get("id")?,
                name: r.try_get("name")?,
                github_url: r.try_get("github_repository_url")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;

    Ok(packages)
}

async fn check_package(pkg: &PackageInfo) -> CheckOutcome {
    let work_dir: PathBuf = std::env::temp_dir().join(format!("noir-compat-{}", pkg.id));
    let _ = tokio::fs::remove_dir_all(&work_dir).await;

    let clone = Command::new("git")
        .args(["clone", "--depth", "1", &pkg.github_url])
        .arg(&work_dir)
        .output()
        .await;

    let clone = match clone {
        Ok(o) => o,
        Err(e) => return CheckOutcome::Error(format!("git spawn failed: {}", e)),
    };

    if !clone.status.success() {
        let stderr = String::from_utf8_lossy(&clone.stderr);
        return CheckOutcome::Error(format!(
            "git clone failed: {}",
            stderr.lines().take(3).collect::<Vec<_>>().join(" | ")
        ));
    }

    if !work_dir.join("Nargo.toml").exists() {
        return CheckOutcome::Error("no Nargo.toml at repo root".to_string());
    }

    let check_fut = Command::new("nargo")
        .arg("check")
        .current_dir(&work_dir)
        .output();

    let check = match tokio::time::timeout(CHECK_TIMEOUT, check_fut).await {
        Ok(Ok(o)) => o,
        Ok(Err(e)) => return CheckOutcome::Error(format!("nargo spawn failed: {}", e)),
        Err(_) => return CheckOutcome::Error(format!("timeout after {}s", CHECK_TIMEOUT.as_secs())),
    };

    if check.status.success() {
        CheckOutcome::Ok
    } else {
        let stderr = String::from_utf8_lossy(&check.stderr);
        let stdout = String::from_utf8_lossy(&check.stdout);
        let combined = if stderr.trim().is_empty() { stdout } else { stderr };
        let snippet: String = combined.lines().take(5).collect::<Vec<_>>().join("\n");
        CheckOutcome::Failed(snippet)
    }
}

async fn record_result(
    pool: &sqlx::PgPool,
    pkg: &PackageInfo,
    nargo_version: &str,
    outcome: &CheckOutcome,
) -> Result<()> {
    let (status, error_snippet) = match outcome {
        CheckOutcome::Ok => ("ok", None),
        CheckOutcome::Failed(s) => ("failed", Some(s.as_str())),
        CheckOutcome::Error(s) => ("error", Some(s.as_str())),
    };

    let error_sql = match error_snippet {
        Some(s) => format!("'{}'", escape_sql_string(s)),
        None => "NULL".to_string(),
    };

    let sql = format!(
        r#"INSERT INTO package_compat_results (package_id, nargo_version, status, error_snippet)
        VALUES ({}, '{}', '{}', {})
        ON CONFLICT (package_id, nargo_version) DO UPDATE SET
            checked_at = NOW(),
            status = EXCLUDED.status,
            error_snippet = EXCLUDED.error_snippet"#,
        pkg.id,
        escape_sql_string(nargo_version),
        status,
        error_sql,
    );

    sqlx::raw_sql(&sql).execute(pool).await?;
    Ok(())
}

fn print_outcome(outcome: &CheckOutcome) {
    match outcome {
        CheckOutcome::Ok => println!("  [ok]"),
        CheckOutcome::Failed(s) => {
            println!("  [failed]");
            for line in s.lines().take(3) {
                println!("    {}", line);
            }
        }
        CheckOutcome::Error(s) => println!("  [error] {}", s),
    }
}
