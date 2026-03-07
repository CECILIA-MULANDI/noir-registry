-- Enable the pg_trgm extension (needed for trigram indexes)
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Trigram index on package name for fast ILIKE search
CREATE INDEX idx_packages_name_trgm ON packages USING gin (name gin_trgm_ops);

-- Trigram index on package description
CREATE INDEX idx_packages_description_trgm ON packages USING gin (description gin_trgm_ops);

-- Trigram index on keywords
CREATE INDEX idx_keywords_keyword_trgm ON package_keywords USING gin (keyword gin_trgm_ops);
