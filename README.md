# Noir Package Registry

A centralized package registry for the Noir programming language ecosystem - like npm for JavaScript or crates.io for Rust.

## Project Goal

Create a package registry that:

- Lists all Noir packages from the ecosystem
- Allows searching and discovering packages
- Provides GraphQL API for package metadata
- Enables future features: publishing, versioning, etc.

## Current Status

### Completed

- [x] Database schema design (PostgreSQL)
- [x] Database setup on Supabase
- [x] Data scraper that fetches packages from awesome-noir
- [x] GitHub API integration for package metadata
- [x] **99 packages** populated in database

### In Progress

- [ ] GraphQL API server
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
│  GraphQL API (Rust + async-graphql) │
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

### Run the API Server (Coming Soon)

```bash
cargo run --bin server
```

This will start the GraphQL API server.

## Project Structure

```
noir-registry/
├── src/
│   ├── main.rs              # API server entry point (coming soon)
│   ├── lib.rs               # Shared library code
│   ├── db/
│   │   └── mod.rs           # Database connection utilities
│   └── bin/
│       └── scraper.rs       # Data scraper binary
├── migrations/
│   └── 20240101000000_initial_schema.sql  # Database schema
├── Cargo.toml               # Rust dependencies
├── .env                     # Environment variables (create this!)
└── README.md                # This file
```

## 🔧 Tech Stack

### Backend

- **Language:** Rust
- **Web Framework:** Axum (coming soon)
- **GraphQL:** async-graphql (coming soon)
- **Database:** SQLx (PostgreSQL driver)
- **HTTP Client:** reqwest

### Database

- **PostgreSQL** via Supabase
- **Migrations:** SQLx CLI

### Frontend (Planned)

- **Framework:** Next.js
- **GraphQL Client:** Apollo Client
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

1. **GraphQL API Server** (Priority 1)

   - Set up Axum web server
   - Create GraphQL schema
   - Implement resolvers for:
     - `packages` query (list all)
     - `package(name)` query (get one)
     - `searchPackages(query)` query

2. **Frontend Website** (Priority 2)

   - Homepage with search bar
   - Package listing page
   - Individual package detail pages
   - Filtering by keywords/tags

3. **Publishing Workflow** (Priority 3)
   - User authentication (GitHub OAuth)
   - Publisher verification
   - Package upload endpoint
   - Version management

### Development Workflow

1. **Create a feature branch**

```bash
git checkout -b feature/graphql-api
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
git commit -m "Add GraphQL API with basic queries"
git push origin feature/graphql-api
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
- [async-graphql Book](https://async-graphql.github.io/async-graphql/en/index.html)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)

## 📞 Questions?

Reach out to the team or open an issue!

---

**Current Stats:** 99 packages indexed and ready to serve! 🎉
