# Noir Package Registry

A centralized package registry for the Noir programming language ecosystem - like npm for JavaScript or crates.io for Rust.

## Project Goal

Create a package registry that:

- Lists all Noir packages from the ecosystem
- Allows searching and discovering packages
- Provides REST API for package metadata
- Enables future features: publishing, versioning, etc.

## Current Status

### Completed

- [x] Database schema design (PostgreSQL)
- [x] Database setup on Supabase
- [x] Data scraper that fetches packages from awesome-noir
- [x] GitHub API integration for package metadata
- [x] REST API server with Axum
- [x] **99 packages** populated in database

### In Progress

- [ ] Frontend web interface
- [ ] Package publishing workflow

## Architecture

```
┌─────────────────────────────────────┐
│  Frontend (Next.js - Coming Soon)  │
│  Package search & discovery UI      │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  REST API (Rust + Axum)             │
│  Query packages, search, filter     │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  PostgreSQL Database (Supabase)     │
│  Stores: packages, versions, tags   │
└─────────────────────────────────────┘
```

## Database Schema

### Tables

**`packages`** - Core package information

- `id` - Primary key
- `name` - Package name (unique)
- `description` - What the package does
- `github_repository_url` - Source code link
- `owner_github_username` - Package author
- `owner_avatar_url` - Author's avatar
- `github_stars` - Popularity metric
- `license` - License type (MIT, Apache, etc.)
- `homepage` - Optional project website
- `total_downloads` - Download count
- `latest_version` - Most recent version
- `created_at`, `updated_at` - Timestamps

**`package_versions`** - Version history (coming soon)

- Links to specific releases of each package

**`package_keywords`** - Tags for filtering

- Many-to-many relationship with packages

## Getting Started

### Prerequisites

- **Rust** (latest stable) - [Install](https://rustup.rs/)
- **PostgreSQL** access (we use Supabase)
- **GitHub Token** (for API rate limits) - [Create one](https://github.com/settings/tokens)

### Installation

1. **Clone the repository**

```bash
git clone https://github.com/CECILIA-MULANDI/noir-registry.git
cd noir-registry
```

2. **Set up environment variables**

Create a `.env` file:

```env
DATABASE_URL=postgresql://postgres.xxx:password@aws-x-xx-xxxx-x.pooler.supabase.com:6543/postgres
GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Get your `DATABASE_URL` from Supabase:

- Go to Project Settings → Database → Connection String
- Copy the URI connection string
- Replace `[YOUR-PASSWORD]` with your actual password

Get your `GITHUB_TOKEN`:

- Go to https://github.com/settings/tokens
- Generate new token (classic)
- Select scope: `public_repo`
- Copy the token

3. **Install dependencies**

```bash
cargo build
```

## Available Commands

### Run the Data Scraper

Fetches packages from awesome-noir and populates the database:

```bash
cargo run --bin scraper
```

**What it does:**

1. Fetches the awesome-noir README from GitHub
2. Parses markdown to extract package names and URLs
3. Calls GitHub API to get metadata (stars, owner, license)
4. Inserts/updates packages in the database

**Expected output:**

```
🚀 Starting Noir package scraper...
🔑 Using GitHub authentication
📦 Connecting to database...
✅ Connected to database!
📥 Fetching awesome-noir README...
✅ Found 103 packages
📡 Fetching GitHub metadata...
✅ Enriched 103 packages
💾 Inserting packages into database...
✅ Inserted 99 packages
```

**Note:** Can be run multiple times safely (uses `ON CONFLICT DO UPDATE`)

### Run the API Server

```bash
cargo run
```

This will start the REST API server on `http://localhost:3000`.

**Available endpoints:**

- `GET /health` - Health check endpoint
- `GET /api/packages` - List all packages (sorted by GitHub stars)
- `GET /api/packages/:name` - Get a specific package by name
- `GET /api/search?q=query` - Search packages by name or description

**Example:**

```bash
# Start the server
cargo run

# In another terminal, test the API
curl http://localhost:3000/health
curl http://localhost:3000/api/packages
curl http://localhost:3000/api/packages/merkle-tree
curl http://localhost:3000/api/search?q=cryptography
```

## Project Structure

```
noir-registry/
├── src/
│   ├── lib.rs                    # Library entry point (shared code)
│   ├── main.rs                   # API server entry point
│   ├── models/                   # Data structures (Package, EnrichedPackage, etc.)
│   │   └── mod.rs
│   ├── github_metadata/          # GitHub API integration
│   │   └── mod.rs
│   ├── package_storage/           # Database operations for packages
│   │   └── mod.rs
│   ├── db/                       # Database connection utilities
│   │   ├── mod.rs
│   │   └── db.rs
│   ├── rest_apis/                # REST API endpoints and handlers
│   │   └── mod.rs
│   └── bin/
│       └── scraper.rs           # Data scraper binary
├── migrations/
│   └── 20251122235733_initial_schema.sql  # Database schema
├── Cargo.toml                   # Rust dependencies
├── .env                         # Environment variables (create this!)
└── README.md                    # This file
```

## 🔧 Tech Stack

### Backend

- **Language:** Rust
- **Web Framework:** Axum
- **API Style:** REST
- **Database:** SQLx (PostgreSQL driver)
- **HTTP Client:** reqwest

### Database

- **PostgreSQL** via Supabase
- **Migrations:** SQLx CLI

### Frontend (Planned)

- **Framework:** Next.js
- **API Client:** Fetch API or Axios
- **Styling:** Tailwind CSS

## Key Concepts

### Why Separate Tables?

**`packages` vs `package_versions`:**

- A package (e.g., "merkle-tree") can have many versions (1.0.0, 1.1.0, 2.0.0)
- Package-level info (name, owner) doesn't change per version
- Version-specific info (dependencies, changelog) does change
- This design matches industry standards (npm, crates.io)

**`package_keywords` (separate table):**

- Enables efficient filtering: "Show me all cryptography packages"
- One package can have many keywords
- Many packages can share the same keyword (many-to-many relationship)

### Data Flow: Scraper

```
awesome-noir README
      ↓
Parse markdown
      ↓
Extract: [name, url, description]
      ↓
For each package:
  Call GitHub API
  Get: owner, stars, license
      ↓
Insert into database
```

## Contributing

### What Needs to Be Built Next

1. **Frontend Website** (Priority 1)

   - Homepage with search bar
   - Package listing page
   - Individual package detail pages
   - Filtering by keywords/tags

2. **Publishing Workflow** (Priority 2)
   - User authentication (GitHub OAuth)
   - Publisher verification
   - Package upload endpoint
   - Version management

### Development Workflow

1. **Create a feature branch**

```bash
git checkout -b feature/rest-api
```

2. **Make your changes**

3. **Test locally**

```bash
cargo test
cargo run --bin scraper  # or --bin server
```

4. **Commit and push**

```bash
git add .
git commit -m "Add rest API with basic queries"
git push origin feature/rest-api
```

5. **Create a Pull Request**

## Troubleshooting

### Scraper Issues

**"403 Forbidden" from GitHub API**

- You've hit the rate limit (60 requests/hour without token)
- Solution: Add `GITHUB_TOKEN` to your `.env` file
- With token: 5,000 requests/hour

**"Connection refused" to database**

- Check your `DATABASE_URL` in `.env`
- Make sure password is correct (no spaces)
- Verify Supabase project is active

**"Prepared statement already exists"**

- This happens with connection poolers
- Use the direct connection string (port 5432, not 6543)
- Or add `?statement_cache_size=0` to URL

### Database Issues

**Tables don't exist**

- Run migrations: `sqlx migrate run`
- Or create tables directly in Supabase SQL Editor

**Can't connect to database**

- Check if DATABASE_URL is set: `echo $DATABASE_URL`
- Export it: `export $(cat .env | xargs)`

## Resources

- [Noir Language Docs](https://noir-lang.org/)
- [awesome-noir Repository](https://github.com/noir-lang/awesome-noir)
- [Supabase Documentation](https://supabase.com/docs)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)

## 📞 Questions?

Reach out to the team or open an issue!

---

**Current Stats:** 99 packages indexed and ready to serve! 🎉
