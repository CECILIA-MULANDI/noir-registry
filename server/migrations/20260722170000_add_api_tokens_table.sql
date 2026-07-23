-- Multi-token model: users own many named tokens, each independently revocable.
-- token_hash is globally unique so validate can find a token from just its raw string.
-- created_at / last_used_at / revoked_at surface useful metadata to users.
-- Backfills any existing users.api_key into a token row named 'legacy' so no user loses access.

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS api_tokens (
    id             SERIAL PRIMARY KEY,
    user_id        INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name           TEXT NOT NULL,
    token_hash     TEXT NOT NULL UNIQUE,
    token_prefix   TEXT NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at   TIMESTAMPTZ,
    revoked_at     TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_api_tokens_user_id ON api_tokens(user_id);

INSERT INTO api_tokens (user_id, name, token_hash, token_prefix)
SELECT id,
       'legacy',
       encode(digest(api_key, 'sha256'), 'hex'),
       substr(api_key, 1, 8)
FROM users
WHERE api_key IS NOT NULL
ON CONFLICT (token_hash) DO NOTHING;
