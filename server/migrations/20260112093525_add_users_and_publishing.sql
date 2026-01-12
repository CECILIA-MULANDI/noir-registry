-- Add migration script here
-- Users authentication table
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    github_id INTEGER UNIQUE NOT NULL,
    github_username TEXT NOT NULL,
    github_avatar_url TEXT,
    api_key TEXT UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_github_id ON users(github_id);
CREATE INDEX idx_users_api_key ON users(api_key);
-- Add publishing fields to packages table
ALTER TABLE packages
    ADD COLUMN IF NOT EXISTS published_by INTEGER REFERENCES users(id),
    ADD COLUMN IF NOT EXISTS source TEXT DEFAULT 'awesome-noir';


-- Update existing packages to mark them as from awesome-noir
UPDATE packages SET source = 'awesome-noir' WHERE source IS NULL;

-- Add index for published_by
CREATE INDEX IF NOT EXISTS idx_packages_published_by ON packages(published_by);