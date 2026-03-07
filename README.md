# Noir Package Registry

A centralized package registry for the Noir programming language ecosystem. Discover, search, and manage Noir packages through the web interface, REST API, or CLI tool.

## Live

| Service | URL |
|---------|-----|
| Web Interface | https://noir-registry.vercel.app |
| API | https://noir-registry-production-229a.up.railway.app |

## Quick Start

**Browse packages:**

Visit the [web interface](https://noir-registry.vercel.app) to explore, search, and filter packages by category.

**Using the API:**

```bash
# List all packages
curl https://noir-registry-production-229a.up.railway.app/api/packages

# Search packages
curl "https://noir-registry-production-229a.up.railway.app/api/search?q=cryptography"

# Get a specific package
curl https://noir-registry-production-229a.up.railway.app/api/packages/package-name

# List categories
curl https://noir-registry-production-229a.up.railway.app/api/categories
```

**Using the CLI tool:**

```bash
# Install
cargo install nargo-add

# Add a package to your Noir project
cd your-noir-project
nargo add package-name

# Remove a package
nargo remove package-name
```

## API Reference

**Base URL:** `https://noir-registry-production-229a.up.railway.app`

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api/packages` | List all packages |
| GET | `/api/packages/:name` | Get package by name |
| GET | `/api/packages?category=slug` | Filter by category |
| GET | `/api/packages?keyword=kw` | Filter by keyword |
| GET | `/api/search?q=query` | Search by name, description, or keyword |
| GET | `/api/categories` | List all categories |
| GET | `/api/keywords` | List all keywords |

## Categories

Packages are organized into 7 categories:

- **Cryptography** — Hashing, encryption, signatures, and crypto primitives
- **Data Structures** — Trees, arrays, sets, and other data structures
- **Math** — Mathematical operations, number theory, and field arithmetic
- **Utilities** — General-purpose helper libraries and tools
- **Zero Knowledge** — ZK proof helpers, verifiers, and proof-system utilities
- **Circuits** — Reusable circuit components and gadgets
- **Standards** — Implementations of standards (EIP, BIP, RFC, etc.)

## CLI Tool

Install the CLI to manage packages directly in your Noir projects:

```bash
cargo install nargo-add
```

**Commands:**

```bash
# Add a package (updates Nargo.toml automatically)
nargo add package-name

# Remove a package
nargo remove package-name

# Remove multiple packages
nargo remove pkg-one pkg-two

# Remove and clean cached source files
nargo remove package-name --clean
```

See [cli-tool/README.md](cli-tool/README.md) for full CLI documentation.

## Local Development

**Requirements:** Rust, Node.js, PostgreSQL (or a Supabase project)

**Backend:**

```bash
cd server
cp .env.example .env   # fill in DATABASE_URL and GITHUB_TOKEN
cargo run
# Runs on http://localhost:3001
```

**Frontend:**

```bash
cd frontend
npm install
npm run dev
# Runs on http://localhost:3000
```

Both must run simultaneously. The frontend proxies `/api/*` to the backend automatically.

**Run migrations:**

```bash
cd server
sqlx migrate run
```

## Tech Stack

- **Backend:** Rust + Axum + SQLx + PostgreSQL
- **Frontend:** Next.js 16 + Tailwind CSS
- **Database:** Supabase (PostgreSQL) with `pg_trgm` indexes for fast search
- **Hosting:** Railway (backend) + Vercel (frontend)

## Resources

- [Noir Language Documentation](https://noir-lang.org/docs)
- [awesome-noir Repository](https://github.com/noir-lang/awesome-noir)

## Contributing

We welcome contributions! Please open an issue or submit a pull request.
