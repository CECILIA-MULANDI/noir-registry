use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub github_id: i32,
    pub github_username: String,
    pub github_avatar_url: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub id: i32,
    pub name: String,
    pub token_prefix: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub id: i32,
    pub login: String,
    pub avatar_url: String,
}

/// Generate a random 32-character API token using the OS CSPRNG.
pub fn generate_api_key() -> String {
    use rand::{Rng, rngs::OsRng};
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const KEY_LEN: usize = 32;

    let mut rng = OsRng;
    (0..KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// SHA-256 hex digest of the raw token. Only the hash is stored; the raw token
/// is shown to the user exactly once at creation and never retrievable again.
pub fn hash_api_key(raw: &str) -> String {
    hex::encode(Sha256::digest(raw.as_bytes()))
}

fn row_to_user(row: sqlx::postgres::PgRow) -> Result<User, sqlx::Error> {
    Ok(User {
        id: row.try_get("id")?,
        github_id: row.try_get("github_id")?,
        github_username: row.try_get("github_username")?,
        github_avatar_url: row.try_get("github_avatar_url")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn row_to_token(row: sqlx::postgres::PgRow) -> Result<ApiToken, sqlx::Error> {
    Ok(ApiToken {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
        token_prefix: row.try_get("token_prefix")?,
        created_at: row.try_get("created_at")?,
        last_used_at: row.try_get("last_used_at")?,
        revoked_at: row.try_get("revoked_at")?,
    })
}

/// Get or create a user from GitHub authentication.
/// Returns the user plus, only when a new user is created, the raw API token
/// for their initial "default" token. Existing users get None because their
/// tokens' raw values aren't recoverable from the stored hashes.
pub async fn get_or_create_user_from_github(
    pool: &PgPool,
    github_token: &str,
) -> Result<(User, Option<String>)> {
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

    // .persistent(false) uses unnamed prepared statements, which pgbouncer transaction mode tolerates.
    let existing = sqlx::query(
        "SELECT id, github_id, github_username, github_avatar_url, created_at, updated_at
         FROM users WHERE github_id = $1",
    )
    .bind(github_user.id)
    .persistent(false)
    .fetch_optional(pool)
    .await?;

    match existing {
        Some(r) => Ok((row_to_user(r)?, None)),
        None => {
            let user_row = sqlx::query(
                "INSERT INTO users (github_id, github_username, github_avatar_url)
                 VALUES ($1, $2, $3)
                 RETURNING id, github_id, github_username, github_avatar_url, created_at, updated_at",
            )
            .bind(github_user.id)
            .bind(&github_user.login)
            .bind(&github_user.avatar_url)
            .persistent(false)
            .fetch_one(pool)
            .await?;
            let user = row_to_user(user_row)?;
            let (_token, raw) = create_token_for_user(pool, user.id, "default").await?;
            Ok((user, Some(raw)))
        }
    }
}

/// Validate a raw token by hashing it and looking up an unrevoked matching row.
/// Returns the owning user, or None if the token is unknown or revoked.
pub async fn validate_api_key(pool: &PgPool, raw_token: &str) -> Result<Option<User>> {
    let token_hash = hash_api_key(raw_token);
    let row = sqlx::query(
        "SELECT u.id, u.github_id, u.github_username, u.github_avatar_url, u.created_at, u.updated_at
         FROM api_tokens t
         JOIN users u ON u.id = t.user_id
         WHERE t.token_hash = $1 AND t.revoked_at IS NULL",
    )
    .bind(&token_hash)
    .persistent(false)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(Some(row_to_user(r)?)),
        None => Ok(None),
    }
}

/// Create a new named token for a user. Returns the token metadata plus the raw
/// string; the caller is responsible for returning the raw string to the user
/// exactly once, because it is never retrievable afterward.
pub async fn create_token_for_user(
    pool: &PgPool,
    user_id: i32,
    name: &str,
) -> Result<(ApiToken, String)> {
    let raw = generate_api_key();
    let token_hash = hash_api_key(&raw);
    let token_prefix: String = raw.chars().take(8).collect();

    let row = sqlx::query(
        "INSERT INTO api_tokens (user_id, name, token_hash, token_prefix)
         VALUES ($1, $2, $3, $4)
         RETURNING id, name, token_prefix, created_at, last_used_at, revoked_at",
    )
    .bind(user_id)
    .bind(name)
    .bind(&token_hash)
    .bind(&token_prefix)
    .persistent(false)
    .fetch_one(pool)
    .await?;

    Ok((row_to_token(row)?, raw))
}

/// List all tokens (including revoked ones) belonging to a user, newest first.
pub async fn list_tokens_for_user(pool: &PgPool, user_id: i32) -> Result<Vec<ApiToken>> {
    let rows = sqlx::query(
        "SELECT id, name, token_prefix, created_at, last_used_at, revoked_at
         FROM api_tokens
         WHERE user_id = $1
         ORDER BY created_at DESC",
    )
    .bind(user_id)
    .persistent(false)
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(|r| row_to_token(r).map_err(Into::into)).collect()
}

/// Revoke a token. Returns true if a row was actually revoked (belonged to the user
/// and wasn't already revoked). Idempotent: revoking twice is a no-op that returns false.
pub async fn revoke_token(pool: &PgPool, user_id: i32, token_id: i32) -> Result<bool> {
    let result = sqlx::query(
        "UPDATE api_tokens
         SET revoked_at = NOW()
         WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL",
    )
    .bind(token_id)
    .bind(user_id)
    .persistent(false)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
