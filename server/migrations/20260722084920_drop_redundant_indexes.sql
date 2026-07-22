-- Drop indexes that duplicate what UNIQUE constraints or composite PKs already provide.
-- Each duplicate B-tree costs disk space and slows writes without helping any query.

DROP INDEX IF EXISTS idx_users_github_id;
DROP INDEX IF EXISTS idx_users_api_key;
DROP INDEX IF EXISTS idx_alternatives_package;
DROP INDEX IF EXISTS idx_packages_name;