use crate::models::{EnrichedPackage, PackageResponse};
use anyhow::Result;
use sqlx::Row;
use std::collections::HashMap;
mod retry;
use retry::retry_on_prepared_statement_error;

/// Escape SQL string for safe interpolation (doubles single quotes)
pub fn escape_sql_string(s: &str) -> String {
    s.replace('\'', "''")
}

/// Format an optional string as SQL: NULL or 'escaped value'
fn sql_opt(opt: &Option<String>) -> String {
    match opt {
        None => "NULL".to_string(),
        Some(s) => format!("'{}'", escape_sql_string(s)),
    }
}

/// Fetches keywords for a batch of package IDs.
/// Returns a map of package_id -> Vec<keyword>.
/// Safe to interpolate: IDs are integers only.
async fn fetch_keywords_map(
    pool: &sqlx::PgPool,
    ids: &[i32],
) -> Result<HashMap<i32, Vec<String>>> {
    if ids.is_empty() {
        return Ok(HashMap::new());
    }
    let ids_str = ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let query = format!(
        "SELECT package_id, keyword FROM package_keywords \
         WHERE package_id IN ({}) ORDER BY keyword",
        ids_str
    );

    let rows = sqlx::raw_sql(&query).fetch_all(pool).await?;

    let mut map: HashMap<i32, Vec<String>> = HashMap::new();
    for row in rows {
        let pkg_id: i32 = row.try_get("package_id")?;
        let keyword: String = row.try_get("keyword")?;
        map.entry(pkg_id).or_default().push(keyword);
    }
    Ok(map)
}

/// Inserts an enriched package into the database
pub async fn insert_package(pool: &sqlx::PgPool, pkg: &EnrichedPackage) -> Result<()> {
    let sql = format!(
        r#"INSERT INTO packages (
            name, description, github_repository_url, homepage, license,
            owner_github_username, owner_avatar_url, github_stars, total_downloads
        ) VALUES ('{}', '{}', '{}', {}, {}, '{}', '{}', {}, 0)
        ON CONFLICT (name) DO UPDATE SET
            description = EXCLUDED.description,
            github_repository_url = EXCLUDED.github_repository_url,
            homepage = EXCLUDED.homepage,
            license = EXCLUDED.license,
            owner_github_username = EXCLUDED.owner_github_username,
            owner_avatar_url = EXCLUDED.owner_avatar_url,
            github_stars = EXCLUDED.github_stars,
            updated_at = CURRENT_TIMESTAMP"#,
        escape_sql_string(&pkg.name),
        escape_sql_string(&pkg.description),
        escape_sql_string(&pkg.github_url),
        sql_opt(&pkg.homepage),
        sql_opt(&pkg.license),
        escape_sql_string(&pkg.owner_username),
        escape_sql_string(&pkg.owner_avatar),
        pkg.stars,
    );
    sqlx::raw_sql(&sql).execute(pool).await?;
    Ok(())
}

/// Retrieves all packages from the database
pub async fn get_all_packages(pool: &sqlx::PgPool) -> Result<Vec<PackageResponse>> {
    retry_on_prepared_statement_error(|| async {
        let rows = sqlx::raw_sql(
            r#"SELECT
                id, name, description, github_repository_url, homepage, license,
                owner_github_username, owner_avatar_url, total_downloads, github_stars,
                latest_version, created_at, updated_at
            FROM packages
            ORDER BY github_stars DESC, name ASC"#,
        )
        .fetch_all(pool)
        .await?;

        let packages: Vec<PackageResponse> = rows
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
                    keywords: vec![],
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        let ids: Vec<i32> = packages.iter().map(|p| p.id).collect();
        let mut keywords_map = fetch_keywords_map(pool, &ids).await?;
        let packages = packages
            .into_iter()
            .map(|mut p| {
                p.keywords = keywords_map.remove(&p.id).unwrap_or_default();
                p
            })
            .collect();

        Ok(packages)
    })
    .await
}

/// Get a single package by name
pub async fn get_package_by_name(
    pool: &sqlx::PgPool,
    name: &str,
) -> Result<Option<PackageResponse>> {
    retry_on_prepared_statement_error(|| async {
        let escaped_name = escape_sql_string(name);
        let query = format!(
            r#"SELECT
                id, name, description, github_repository_url, homepage, license,
                owner_github_username, owner_avatar_url, total_downloads, github_stars,
                latest_version, created_at, updated_at
            FROM packages WHERE name = '{}'"#,
            escaped_name
        );

        let row = sqlx::raw_sql(&query).fetch_all(pool).await?.into_iter().next();

        match row {
            Some(row) => {
                let mut pkg = PackageResponse {
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
                    keywords: vec![],
                };
                let mut map = fetch_keywords_map(pool, &[pkg.id]).await?;
                pkg.keywords = map.remove(&pkg.id).unwrap_or_default();
                Ok(Some(pkg))
            }
            None => Ok(None),
        }
    })
    .await
}

/// Search packages by name, description, or keywords
pub async fn search_packages(pool: &sqlx::PgPool, query: &str) -> Result<Vec<PackageResponse>> {
    retry_on_prepared_statement_error(|| async {
        let escaped_query = escape_sql_string(query);
        let search_pattern = format!("%{}%", escaped_query);
        let search_prefix = format!("{}%", escaped_query);

        let sql_query = format!(
            r#"SELECT DISTINCT
                p.id, p.name, p.description, p.github_repository_url, p.homepage, p.license,
                p.owner_github_username, p.owner_avatar_url, p.total_downloads, p.github_stars,
                p.latest_version, p.created_at, p.updated_at
            FROM packages p
            LEFT JOIN package_keywords pk ON p.id = pk.package_id
            WHERE
                p.name ILIKE '{pat}'
                OR p.description ILIKE '{pat}'
                OR pk.keyword ILIKE '{pat}'
            ORDER BY
                CASE
                    WHEN p.name ILIKE '{prefix}' THEN 1
                    WHEN p.description ILIKE '{prefix}' THEN 2
                    ELSE 3
                END,
                p.github_stars DESC,
                p.name ASC"#,
            pat = search_pattern,
            prefix = search_prefix
        );

        let rows = sqlx::raw_sql(&sql_query).fetch_all(pool).await?;

        let packages: Vec<PackageResponse> = rows
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
                    keywords: vec![],
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        let ids: Vec<i32> = packages.iter().map(|p| p.id).collect();
        let mut keywords_map = fetch_keywords_map(pool, &ids).await?;
        let packages = packages
            .into_iter()
            .map(|mut p| {
                p.keywords = keywords_map.remove(&p.id).unwrap_or_default();
                p
            })
            .collect();

        Ok(packages)
    })
    .await
}

/// Get packages filtered by a specific keyword
pub async fn get_packages_by_keyword(
    pool: &sqlx::PgPool,
    keyword: &str,
) -> Result<Vec<PackageResponse>> {
    let escaped = escape_sql_string(keyword);
    let query = format!(
        r#"SELECT
            p.id, p.name, p.description, p.github_repository_url,
            p.homepage, p.license, p.owner_github_username, p.owner_avatar_url,
            p.total_downloads, p.github_stars, p.latest_version,
            p.created_at, p.updated_at
        FROM packages p
        INNER JOIN package_keywords pk ON p.id = pk.package_id
        WHERE pk.keyword = '{}'
        ORDER BY p.github_stars DESC, p.name ASC"#,
        escaped
    );

    let rows = sqlx::raw_sql(&query).fetch_all(pool).await?;

    let packages: Vec<PackageResponse> = rows
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
                keywords: vec![],
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;

    let ids: Vec<i32> = packages.iter().map(|p| p.id).collect();
    let mut keywords_map = fetch_keywords_map(pool, &ids).await?;
    let packages = packages
        .into_iter()
        .map(|mut p| {
            p.keywords = keywords_map.remove(&p.id).unwrap_or_default();
            p
        })
        .collect();

    Ok(packages)
}

/// Get all unique keywords in the registry
pub async fn get_all_keywords(pool: &sqlx::PgPool) -> Result<Vec<String>> {
    let rows = sqlx::raw_sql(
        "SELECT DISTINCT keyword FROM package_keywords ORDER BY keyword",
    )
    .fetch_all(pool)
    .await?;

    let keywords = rows
        .into_iter()
        .map(|row| row.try_get::<String, _>("keyword").map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>>>()?;

    Ok(keywords)
}

/// Insert or replace keywords for a package
pub async fn save_keywords(
    pool: &sqlx::PgPool,
    package_id: i32,
    keywords: &[String],
) -> Result<()> {
    let delete_query = format!(
        "DELETE FROM package_keywords WHERE package_id = {}",
        package_id
    );
    sqlx::raw_sql(&delete_query).execute(pool).await?;

    for keyword in keywords {
        let kw = keyword.trim().to_lowercase();
        if kw.is_empty() {
            continue;
        }
        let escaped_kw = escape_sql_string(&kw);
        let insert_query = format!(
            "INSERT INTO package_keywords (package_id, keyword) \
             VALUES ({}, '{}') ON CONFLICT DO NOTHING",
            package_id, escaped_kw
        );
        sqlx::raw_sql(&insert_query).execute(pool).await?;
    }

    Ok(())
}

/// Increment the download counter for a package by name
pub async fn increment_downloads(pool: &sqlx::PgPool, name: &str) -> Result<()> {
    let escaped = escape_sql_string(name);
    let query = format!(
        "UPDATE packages SET total_downloads = total_downloads + 1 WHERE name = '{}'",
        escaped
    );
    sqlx::raw_sql(&query).execute(pool).await?;
    Ok(())
}

/// --- Category helpers ---

/// Get all categories
pub async fn get_all_categories(pool: &sqlx::PgPool) -> Result<Vec<crate::models::Category>> {
    let rows = sqlx::raw_sql(
        "SELECT id, name, slug, description FROM categories ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await?;

    let categories = rows
        .into_iter()
        .map(|row| {
            Ok(crate::models::Category {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                slug: row.try_get("slug")?,
                description: row.try_get("description")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;

    Ok(categories)
}

/// Get packages filtered by category slug
pub async fn get_packages_by_category(
    pool: &sqlx::PgPool,
    slug: &str,
) -> Result<Vec<PackageResponse>> {
    let escaped = escape_sql_string(slug);
    let query = format!(
        r#"SELECT
            p.id, p.name, p.description, p.github_repository_url,
            p.homepage, p.license, p.owner_github_username, p.owner_avatar_url,
            p.total_downloads, p.github_stars, p.latest_version,
            p.created_at, p.updated_at
        FROM packages p
        INNER JOIN package_categories pc ON p.id = pc.package_id
        INNER JOIN categories c ON pc.category_id = c.id
        WHERE c.slug = '{}'
        ORDER BY p.github_stars DESC, p.name ASC"#,
        escaped
    );

    let rows = sqlx::raw_sql(&query).fetch_all(pool).await?;

    let packages: Vec<PackageResponse> = rows
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
                keywords: vec![],
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;

    let ids: Vec<i32> = packages.iter().map(|p| p.id).collect();
    let mut keywords_map = fetch_keywords_map(pool, &ids).await?;
    let packages = packages
        .into_iter()
        .map(|mut p| {
            p.keywords = keywords_map.remove(&p.id).unwrap_or_default();
            p
        })
        .collect();

    Ok(packages)
}

/// Assign a category to a package (by category slug)
pub async fn save_package_category(
    pool: &sqlx::PgPool,
    package_id: i32,
    category_slug: &str,
) -> Result<()> {
    let escaped_slug = escape_sql_string(category_slug);
    let query = format!(
        "SELECT id FROM categories WHERE slug = '{}'",
        escaped_slug
    );
    let row = sqlx::raw_sql(&query).fetch_all(pool).await?.into_iter().next();

    let category_id: i32 = match row {
        Some(r) => r.try_get("id")?,
        None => anyhow::bail!("Category '{}' not found", category_slug),
    };

    let delete_query = format!(
        "DELETE FROM package_categories WHERE package_id = {}",
        package_id
    );
    sqlx::raw_sql(&delete_query).execute(pool).await?;

    let insert_query = format!(
        "INSERT INTO package_categories (package_id, category_id) VALUES ({}, {}) ON CONFLICT DO NOTHING",
        package_id, category_id
    );
    sqlx::raw_sql(&insert_query).execute(pool).await?;

    Ok(())
}

/// Get category for a package
pub async fn get_package_category(
    pool: &sqlx::PgPool,
    package_id: i32,
) -> Result<Option<crate::models::Category>> {
    let query = format!(
        r#"SELECT c.id, c.name, c.slug, c.description
        FROM categories c
        INNER JOIN package_categories pc ON c.id = pc.category_id
        WHERE pc.package_id = {}"#,
        package_id
    );
    let row = sqlx::raw_sql(&query).fetch_all(pool).await?.into_iter().next();

    match row {
        Some(r) => Ok(Some(crate::models::Category {
            id: r.try_get("id")?,
            name: r.try_get("name")?,
            slug: r.try_get("slug")?,
            description: r.try_get("description")?,
        })),
        None => Ok(None),
    }
}
