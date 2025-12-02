# nargo-add CLI Tool

A CLI tool to add packages from the Noir registry to your `Nargo.toml`.

## Installation

### Install from crates.io (Recommended) ⭐

```bash
cargo install nargo-add
```

This installs `nargo-add` to `~/.cargo/bin/`. Make sure `~/.cargo/bin` is in your PATH.

### Install from GitHub

```bash
cargo install --git https://github.com/CECILIA-MULANDI/noir-registry --bin nargo-add --path cli-tool
```

### Configure Registry URL (Optional)

The tool defaults to `http://109.205.177.65/api`. To use a different registry:

```bash
# Set environment variable (in your ~/.bashrc or ~/.zshrc)
export NOIR_REGISTRY_URL="https://your-registry.com/api"
```

Or use the `--registry` flag for each command.

### Alternative: Build and Install Manually

```bash
# Build release binary
cd noir-registry
cargo build --release -p nargo-add

# Copy to a directory in your PATH
cp target/release/nargo-add ~/.cargo/bin/
# or
cp target/release/nargo-add ~/.local/bin/
```

## Usage

Once installed, use `nargo-add` in your Noir project:

```bash
# Navigate to your Noir project
cd my-noir-project

# Add a package from the registry
nargo-add rocq-of-noir

# Add with custom registry URL
nargo-add rocq-of-noir --registry http://your-registry.com/api

# Add with specific Nargo.toml path
nargo-add rocq-of-noir --manifest-path /path/to/Nargo.toml
```

## Example Workflow

```bash
# 1. Create a new Noir project (or navigate to existing one)
nargo new my-project
cd my-project

# 2. Add dependencies from the registry
nargo-add rocq-of-noir
nargo-add ECrecover

# 3. Your Nargo.toml now has the dependencies
cat Nargo.toml
```

## How it works

- Fetches package information from your registry API
- Finds `Nargo.toml` in the current directory (or walks up to find it)
- Adds the dependency with the correct format: `package-name = { git = "url" }`

## Requirements

- Rust and Cargo installed
- Your registry server should be running
- Network access to the registry API

## Configuration

### Environment Variables

- `NOIR_REGISTRY_URL` - Default registry API URL (defaults to `http://109.205.177.65/api`)

Example:
```bash
export NOIR_REGISTRY_URL="http://109.205.177.65/api"
nargo-add rocq-of-noir
```

### Command Line Options

- `--registry <URL>` - Override registry URL for this command
- `--manifest-path <PATH>` - Specify Nargo.toml path explicitly

## Features

✅ **Production-Ready Features:**
- Environment variable support (`NOIR_REGISTRY_URL`)
- Network retry logic with exponential backoff
- Comprehensive error messages with troubleshooting tips
- TOML validation after modifications
- Timeout handling for network requests
- Clear user feedback and progress indicators

## Troubleshooting

**"nargo-add: command not found"**
- Install from crates.io: `cargo install nargo-add`
- Or install from GitHub: `cargo install --git https://github.com/CECILIA-MULANDI/noir-registry --bin nargo-add --path cli-tool`
- Check that `~/.cargo/bin` is in your PATH: `echo $PATH`
- Verify installation: `which nargo-add`

**"Could not find Nargo.toml"**
- Make sure you're in a Noir project directory
- Or use `--manifest-path` to specify the path: `nargo-add package-name --manifest-path /path/to/Nargo.toml`

**"Package not found in registry"**
- Make sure your registry server is running
- Check the registry URL: `echo $NOIR_REGISTRY_URL` or use `--registry` flag
- Verify the package name exists: `curl $NOIR_REGISTRY_URL/packages/package-name`
- Check network connectivity

**"Failed to connect to registry"**
- The tool will automatically retry 3 times with exponential backoff
- Check that the registry URL is correct
- Verify network connectivity: `curl $NOIR_REGISTRY_URL/health`
- Check firewall/proxy settings

**"Dependency already exists"**
- The tool prevents duplicate dependencies
- To update, manually edit Nargo.toml or remove and re-add

