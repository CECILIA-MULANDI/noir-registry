-- Fix timestamp columns to use TIMESTAMPTZ instead of TIMESTAMP
-- This matches the Rust chrono::DateTime<Utc> type expectations

ALTER TABLE packages 
    ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';

ALTER TABLE package_versions
    ALTER COLUMN published_at TYPE TIMESTAMPTZ USING published_at AT TIME ZONE 'UTC';

