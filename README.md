# Noir Package Registry

A centralized package registry for the Noir programming language ecosystem. Discover, search, and manage Noir packages through our web interface, REST API, or CLI tool.

## Quick Start

**Using the API:**

```bash
# List all packages
curl http://109.205.177.65/api/packages

# Search packages
curl "http://109.205.177.65/api/search?q=cryptography"

# Get specific package
curl http://109.205.177.65/api/packages/CodeTracer
```

**Using the CLI tool:**

```bash
# Install
cargo install nargo-add

# Use in your Noir project
cd your-noir-project
nargo add package-name
```

## Live Services

### Web Interface

**Frontend:** [https://noir-registry.vercel.app/](https://noir-registry.vercel.app/)

Browse packages, search, and explore the Noir package ecosystem through the web interface.

### API Server

**Base URL:** `http://109.205.177.65`

**Available Endpoints:**

- `GET /health` - Health check
- `GET /api/packages` - List all packages
- `GET /api/packages/:name` - Get package by name
- `GET /api/search?q=query` - Search packages

**Try it:**

```bash
curl http://109.205.177.65/health
curl http://109.205.177.65/api/packages | head -20
curl "http://109.205.177.65/api/search?q=cryptography"
```

## CLI Tool: nargo add

Install the CLI tool to easily add packages to your Noir projects:

```bash
cargo install nargo-add
```

**Usage:**

```bash
cd your-noir-project
nargo add package-name
```

The tool automatically fetches package info from the registry and adds it to your `Nargo.toml`. After installation, you can use `nargo add` directly - it works seamlessly with your existing `nargo` installation. See [cli-tool/README.md](cli-tool/README.md) for more details.

## Frontend Web Interface

The frontend is built with Next.js and deployed at **[https://noir-registry.vercel.app/](https://noir-registry.vercel.app/)**.

**Features:**

- Browse all packages
- Search functionality
- Package detail pages
- Responsive design with Tailwind CSS

Visit the [web interface](https://noir-registry.vercel.app/) to explore packages visually.

## Statistics

- **99 packages** indexed and available
- **Web interface** live at [https://noir-registry.vercel.app/](https://noir-registry.vercel.app/)
- **API** live at `http://109.205.177.65`
- **CLI tool** available: `cargo install nargo-add`

## Resources

- [Noir Language Documentation](https://noir-lang.org/)
- [awesome-noir Repository](https://github.com/noir-lang/awesome-noir)

## Contributing

We welcome contributions! Please open an issue or submit a pull request.
