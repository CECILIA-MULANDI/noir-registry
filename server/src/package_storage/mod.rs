use crate::models::{EnrichedPackage, PackageResponse};
use anyhow::Result;
use sqlx::Row;
mod retry;
use retry::retry_on_prepared_statement_error;
/// Escape SQL string for safe interpolation (doubles single quotes)
fn escape_sql_string(s: &str) -> String {
    s.replace('\'', "''")
}
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
// Retrieves all packages from the database
pub async fn get_all_packages(pool: &sqlx::PgPool) -> Result<Vec<PackageResponse>> {
    // Wrap the query in retry logic to handle prepared statement cache conflicts
    retry_on_prepared_statement_error(|| async {
        // Use raw SQL execution to avoid PREPARE commands entirely
        // This is the ONLY way to avoid prepared statements with PgBouncer transaction mode
        // Using pool directly with persistent(false) to avoid statement caching
        let rows = sqlx::query(
            r#"
            SELECT 
                id,
                name,
                description,
                github_repository_url,
                homepage,
                license,
                owner_github_username,
                owner_avatar_url,
                total_downloads,
                github_stars,
                latest_version,
                created_at,
                updated_at
            FROM packages
            ORDER BY github_stars DESC, name ASC
            "#,
        )
        .persistent(false)
        .fetch_all(pool)
        .await?;

        // Manually extract fields to avoid any prepared statement usage
        let packages: Result<Vec<PackageResponse>, sqlx::Error> = rows
            .into_iter()
            .map(|row| {
                Ok(PackageResponse {
                    id: row.try_get("id")?,
                    name: row.try_get("name")?,
                    description: row.try_get("description")?,
                    github_repository_url: row.try_get("github_repository_url")?,
                    homepage: row.try_get("homepage")?,
                    license: row.try_get("license")?,
                    owner_github_username: row.try_get("owner_github_username")?,
                    owner_avatar_url: row.try_get("owner_avatar_url")?,
                    total_downloads: row.try_get("total_downloads")?,
                    github_stars: row.try_get("github_stars")?,
                    latest_version: row.try_get("latest_version")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                })
            })
            .collect();

        Ok(packages?)
    })
    .await
}

/// Get a single package by name
pub async fn get_package_by_name(
    pool: &sqlx::PgPool,
    name: &str,
) -> Result<Option<PackageResponse>> {
    retry_on_prepared_statement_error(|| async {
        // Use string interpolation to avoid prepared statements (required for PgBouncer transaction mode)
        let escaped_name = escape_sql_string(name);
        let query = format!(
            r#"
            SELECT 
                id,
                name,
                description,
                github_repository_url,
                homepage,
                license,
                owner_github_username,
                owner_avatar_url,
                total_downloads,
                github_stars,
                latest_version,
                created_at,
                updated_at
            FROM packages
            WHERE name = '{}'
            "#,
            escaped_name
        );

        let row = sqlx::query(&query)
            .persistent(false) // Disable prepared statement caching for PgBouncer compatibility
            .fetch_optional(pool)
            .await?;

        match row {
            Some(row) => Ok(Some(PackageResponse {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                description: row.try_get("description")?,
                github_repository_url: row.try_get("github_repository_url")?,
                homepage: row.try_get("homepage")?,
                license: row.try_get("license")?,
                owner_github_username: row.try_get("owner_github_username")?,
                owner_avatar_url: row.try_get("owner_avatar_url")?,
                total_downloads: row.try_get("total_downloads")?,
                github_stars: row.try_get("github_stars")?,
                latest_version: row.try_get("latest_version")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })),
            None => Ok(None),
        }
    })
    .await
}

/// Search packages by name or description
pub async fn search_packages(pool: &sqlx::PgPool, query: &str) -> Result<Vec<PackageResponse>> {
    retry_on_prepared_statement_error(|| async {
        // Use string interpolation to avoid prepared statements (required for PgBouncer transaction mode)
        let escaped_query = escape_sql_string(query);
        let search_pattern = format!("%{}%", escaped_query);
        let search_prefix = format!("{}%", escaped_query);

        let sql_query = format!(
            r#"
            SELECT 
                id,
                name,
                description,
                github_repository_url,
                homepage,
                license,
                owner_github_username,
                owner_avatar_url,
                total_downloads,
                github_stars,
                latest_version,
                created_at,
                updated_at
            FROM packages
            WHERE 
                name ILIKE '{}' 
                OR description ILIKE '{}'
            ORDER BY 
                CASE 
                    WHEN name ILIKE '{}' THEN 1
                    WHEN description ILIKE '{}' THEN 2
                    ELSE 3
                END,
                github_stars DESC,
                name ASC
            "#,
            search_pattern, search_pattern, search_prefix, search_prefix
        );

        let rows = sqlx::query(&sql_query)
            .persistent(false) // Disable prepared statement caching for PgBouncer compatibility
            .fetch_all(pool)
            .await?;

        // Manually extract fields to avoid any prepared statement usage
        let packages: Result<Vec<PackageResponse>, sqlx::Error> = rows
            .into_iter()
            .map(|row| {
                Ok(PackageResponse {
                    id: row.try_get("id")?,
                    name: row.try_get("name")?,
                    description: row.try_get("description")?,
                    github_repository_url: row.try_get("github_repository_url")?,
                    homepage: row.try_get("homepage")?,
                    license: row.try_get("license")?,
                    owner_github_username: row.try_get("owner_github_username")?,
                    owner_avatar_url: row.try_get("owner_avatar_url")?,
                    total_downloads: row.try_get("total_downloads")?,
                    github_stars: row.try_get("github_stars")?,
                    latest_version: row.try_get("latest_version")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                })
            })
            .collect();

        Ok(packages?)
    })
    .await
}

