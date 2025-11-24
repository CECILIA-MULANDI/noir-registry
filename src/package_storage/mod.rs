use anyhow::Result;
use crate::models::EnrichedPackage;
/// Inserts an enriched package into the database
pub async fn insert_package(pool: &sqlx::PgPool, pkg: &EnrichedPackage) -> Result<()> {
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
