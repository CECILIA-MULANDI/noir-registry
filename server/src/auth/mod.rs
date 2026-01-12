use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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
/// Generate a random API key(32 characters)
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
/// Get or create a user from Github authentication

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
    let user = sqlx::query_as!(
        User,
        "SELECT id, github_id, github_username, github_avatar_url, api_key, created_at, updated_at
         FROM users WHERE github_id = $1",
        github_user.id
    )
    .fetch_optional(pool)
    .await?;

    match user {
        // User exists - return them
        Some(u) => Ok(u),
        // User doesn't exist - create them
        None => {
            // Generate a new API key for this user
            let api_key = generate_api_key();

            // Insert into database
            let user = sqlx::query_as!(
                User,
                "INSERT INTO users (github_id, github_username, github_avatar_url, api_key)
                 VALUES ($1, $2, $3, $4)
                 RETURNING id, github_id, github_username, github_avatar_url, api_key, created_at, updated_at",
                github_user.id,
                github_user.login,
                github_user.avatar_url,
                api_key
            )
            .fetch_one(pool)
            .await?;

            Ok(user)
        }
    }
}

/// Validate an API key and return the associated user
pub async fn validate_api_key(pool: &PgPool, api_key: &str) -> Result<Option<User>> {
    // Look up the user by their API key
    let user = sqlx::query_as!(
        User,
        "SELECT id, github_id, github_username, github_avatar_url, api_key, created_at, updated_at
         FROM users WHERE api_key = $1",
        api_key
    )
    .fetch_optional(pool)
    .await?;
    // Returns Some(user) if key is valid, None if invalid
    Ok(user)
}
