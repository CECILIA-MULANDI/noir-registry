-- Add migration script here
-- This table keeps track of the details about a package
-- We include metrics like how many download it has among others
CREATE TABLE packages(
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    github_repository_url TEXT NOT NULL,
    homepage TEXT,
    license TEXT,
    -- some info about the owner of the package
    owner_github_username TEXT NOT NULL,
    owner_avatar_url TEXT,

    --Metric about the package
    total_downloads INTEGER DEFAULT 0,
    github_stars INTEGER DEFAULT 0,

    --latest version of the package tracking
    latest_version TEXT,
    latest_version_id INTEGER,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- This table keeps track of all the package versions that 
-- a user has ever uploaded
CREATE TABLE package_versions(
    id SERIAL PRIMARY KEY,
    package_id INTEGER NOT NULL,
    version TEXT NOT NULL,
    readme TEXT,
    changelog TEXT,
    noir_version_requirement TEXT,
    download_url TEXT,
    checksum TEXT,
    file_size INTEGER,
    downloads INTEGER DEFAULT 0,
    published_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE,
    UNIQUE(package_id, version)

);
-- Keywords table
CREATE TABLE package_keywords (
    package_id INTEGER NOT NULL,
    keyword TEXT NOT NULL,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE,
    PRIMARY KEY (package_id, keyword)
);

-- Indexes for performance
CREATE INDEX idx_packages_name ON packages(name);
CREATE INDEX idx_packages_owner ON packages(owner_github_username);
CREATE INDEX idx_packages_stars ON packages(github_stars DESC);
CREATE INDEX idx_packages_downloads ON packages(total_downloads DESC);
CREATE INDEX idx_keywords_keyword ON package_keywords(keyword);
CREATE INDEX idx_versions_package ON package_versions(package_id);