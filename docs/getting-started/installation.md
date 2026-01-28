# Installation

## From Source (Cargo)

qstorm requires Rust 1.75 or later.

```bash
# Clone the repository
git clone https://github.com/nathanleroy/qstorm
cd qstorm

# Build with all features
cargo build --release --all-features

# Or install globally
cargo install --path qstorm-cli --features all-providers,embeddings
```

## Feature Flags

qstorm uses feature flags to control which providers are compiled:

| Feature | Description |
|---------|-------------|
| `elasticsearch` | Enable Elasticsearch provider |
| `qdrant` | Enable Qdrant provider |
| `embeddings` | Enable fastembed for text-to-vector conversion |
| `all-providers` | Enable all provider features |

### Minimal Build

If you only need Qdrant support:

```bash
cargo build --release --features qdrant,embeddings
```

### Without Embeddings

If you have pre-computed vectors, you can skip the embeddings feature:

```bash
cargo build --release --features qdrant
```

Note: Without embeddings, you'll need to provide vectors directly in your query file.

## Verify Installation

```bash
qstorm --help
```

You should see:

```
Vector search load testing tool

Usage: qstorm [OPTIONS] --queries <QUERIES>

Options:
  -c, --config <CONFIG>    Path to configuration file [default: qstorm.yaml]
  -q, --queries <QUERIES>  Path to queries file
      --headless           Run in headless mode
  -b, --bursts <BURSTS>    Number of bursts to run [default: 0]
      --output <OUTPUT>    Output format [default: json]
  -h, --help               Print help
```