use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub github_id: i32,
    pub github_username: String,
    pub github_avatar_url: Option<String>,
    pub api_key: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub id: i32,
    pub login: String,
    pub avatar_url: String,
}

/// Generate a random API key (32 characters)
pub fn generate_api_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const KEY_LEN: usize = 32;

    let mut rng = rand::thread_rng();
    (0..KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn escape_sql(s: &str) -> String {
    s.replace('\'', "''")
}

fn row_to_user(row: sqlx::postgres::PgRow) -> Result<User, sqlx::Error> {
    Ok(User {
        id: row.try_get("id")?,
        github_id: row.try_get("github_id")?,
        github_username: row.try_get("github_username")?,
        github_avatar_url: row.try_get("github_avatar_url")?,
        api_key: row.try_get("api_key")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

/// Get or create a user from GitHub authentication
pub async fn get_or_create_user_from_github(pool: &PgPool, github_token: &str) -> Result<User> {
    let client = reqwest::Client::new();
    let github_user: GithubUser = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "noir-registry")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?
        .json()
        .await?;

    // github_id is i32 — safe to format directly without quoting
    let find_sql = format!(
        "SELECT id, github_id, github_username, github_avatar_url, api_key, created_at, updated_at
         FROM users WHERE github_id = {}",
        github_user.id
    );
    let row = sqlx::raw_sql(&find_sql).fetch_all(pool).await?.into_iter().next();

    match row {
        Some(r) => Ok(row_to_user(r)?),
        None => {
            let api_key = generate_api_key();
            let insert_sql = format!(
                "INSERT INTO users (github_id, github_username, github_avatar_url, api_key)
                 VALUES ({}, '{}', '{}', '{}')
                 RETURNING id, github_id, github_username, github_avatar_url, api_key, created_at, updated_at",
                github_user.id,
                escape_sql(&github_user.login),
                escape_sql(&github_user.avatar_url),
                escape_sql(&api_key),
            );
            let row = sqlx::raw_sql(&insert_sql).fetch_one(pool).await?;
            Ok(row_to_user(row)?)
        }
    }
}

/// Validate an API key and return the associated user
pub async fn validate_api_key(pool: &PgPool, api_key: &str) -> Result<Option<User>> {
    let sql = format!(
        "SELECT id, github_id, github_username, github_avatar_url, api_key, created_at, updated_at
         FROM users WHERE api_key = '{}'",
        escape_sql(api_key)
    );
    let row = sqlx::raw_sql(&sql).fetch_all(pool).await?.into_iter().next();

    match row {
        Some(r) => Ok(Some(row_to_user(r)?)),
        None => Ok(None),
    }
}
