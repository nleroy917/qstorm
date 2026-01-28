# qstorm

Vector search load testing tool.

## Installation

```bash
cargo install --path qstorm-cli --features all-providers,embeddings
```

## Usage

1. Create a config file (`qstorm.yaml`):

```yaml
provider:
  name: "my-qdrant"
  type: qdrant
  url: "http://localhost:6333"
  index: "my-collection"
  vector_field: "vector"

benchmark:
  burst_size: 100
  concurrency: 10

embedding:
  model: "BAAI/bge-small-en-v1.5"
```

2. Create a queries file (`queries.yaml`):

```yaml
queries:
  - "wireless headphones"
  - "ergonomic chair"
  - "mechanical keyboard"
```

3. Run:

```bash
# Interactive TUI
qstorm -c qstorm.yaml -q queries.yaml

# Headless (for CI)
qstorm -c qstorm.yaml -q queries.yaml --headless --bursts 10
```

## License

MIT