# nargo-add CLI Tool

A CLI tool to add packages from the Noir registry to your `Nargo.toml`.

## Installation

### Quick Install (Recommended)

```bash
# From the noir-registry directory
cd noir-registry
cargo install --path cli-tool --bin nargo-add

# This installs nargo-add to ~/.cargo/bin/
# Make sure ~/.cargo/bin is in your PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

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
- Your registry server should be running (default: http://localhost:8080/api)

## Troubleshooting

**"nargo-add: command not found"**
- Make sure `nargo-add` is installed: `cargo install --path cli-tool --bin nargo-add`
- Check that `~/.cargo/bin` is in your PATH: `echo $PATH`
- Verify installation: `which nargo-add`

**"Could not find Nargo.toml"**
- Make sure you're in a Noir project directory
- Or use `--manifest-path` to specify the path: `nargo-add package-name --manifest-path /path/to/Nargo.toml`

**"Package not found in registry"**
- Make sure your registry server is running
- Check the registry URL with `--registry` flag: `nargo-add package-name --registry http://your-registry.com/api`
- Verify the package name exists in your registry

