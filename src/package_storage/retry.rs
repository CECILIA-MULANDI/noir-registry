use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

/// Retries a database operation if it fails due to prepared statement cache issues
/// This handles the PgBouncer "prepared statement already exists" error gracefully
pub async fn retry_on_prepared_statement_error<F, Fut, T>(mut operation: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    const MAX_RETRIES: u32 = 5;
    // Longer delays since cache needs time to clear: 500ms, 1s, 2s, 4s, 8s
    const INITIAL_DELAY_MS: u64 = 500;
    
    for attempt in 0..=MAX_RETRIES {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                let error_msg = e.to_string();
                
                // Check if it's a prepared statement error
                if error_msg.contains("prepared statement") && error_msg.contains("already exists") {
                    if attempt < MAX_RETRIES {
                        // Exponential backoff with longer delays: 500ms, 1s, 2s, 4s, 8s
                        let delay_ms = INITIAL_DELAY_MS * (1 << attempt);
                        let delay_secs = delay_ms as f64 / 1000.0;
                        eprintln!("⚠️  Prepared statement cache conflict (attempt {}/{}), retrying in {:.1}s...", 
                                 attempt + 1, MAX_RETRIES + 1, delay_secs);
                        sleep(Duration::from_millis(delay_ms)).await;
                        continue;
                    } else {
                        // Last attempt failed - this shouldn't happen if using direct connection
                        eprintln!("❌ Prepared statement error persisted after {} retries", MAX_RETRIES + 1);
                        eprintln!("   This usually means you're using PgBouncer pooler (port 6543)");
                        eprintln!("   The server will auto-switch to direct connection (port 5432) on next restart");
                        eprintln!("   Or manually change your DATABASE_URL from :6543 to :5432");
                        return Err(e);
                    }
                } else {
                    // Not a prepared statement error - return immediately
                    return Err(e);
                }
            }
        }
    }
    
    unreachable!()
}

