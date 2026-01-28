# Providers

qstorm supports multiple vector search engines through a pluggable provider system.

## Supported Providers

| Provider | Status | Feature Flag |
|----------|--------|--------------|
| [Qdrant](qdrant.md) | Supported | `qdrant` |
| [Elasticsearch](elasticsearch.md) | Supported | `elasticsearch` |
| OpenSearch | Planned | - |
| Weaviate | Planned | - |
| Pinecone | Planned | - |
| Milvus | Planned | - |

## Feature Flags

Providers are gated behind Cargo feature flags to minimize binary size and dependencies.

```bash
# Build with specific providers
cargo build --release --features qdrant,embeddings

# Build with all providers
cargo build --release --features all-providers,embeddings
```

## Common Configuration

All providers share these configuration options:

```yaml
provider:
  name: "display-name"      # Required - shown in UI
  type: provider-type       # Required - qdrant, elasticsearch, etc.
  url: "http://..."         # Required - connection URL
  index: "index-name"       # Required - index/collection name
  vector_field: "vector"    # Optional - default: "vector"
  credentials:              # Optional - authentication
    type: basic|api_key|bearer
    # ... type-specific fields
```

## Adding New Providers

qstorm uses a trait-based architecture. To add a new provider:

1. Implement `SearchProvider` trait in `qstorm-core/src/providers/`
2. Add feature flag to `Cargo.toml`
3. Register in `providers/mod.rs`
4. Add to CLI provider factory

See existing implementations for reference.

## Provider Capabilities

Each provider reports its capabilities:

```rust
Capabilities {
    vector_search: true,
    vector_dimension: Some(384),
}
```

Currently qstorm focuses on vector search. Keyword and hybrid search support is planned for future releases.