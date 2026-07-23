-- Apply this ONLY after the multi-token code is deployed and stable.
-- Removes the legacy single-key column from users; tokens now live in api_tokens.

ALTER TABLE users DROP COLUMN IF EXISTS api_key;
