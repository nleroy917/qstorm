# qstorm

**Vector search load testing tool**

qstorm is a high-performance benchmarking tool for vector search engines. It helps you measure latency, throughput, and performance characteristics of your vector database under load.

## Features

- **Multiple Providers** - Support for Qdrant, Elasticsearch, and more
- **Automatic Embedding** - Converts text queries to vectors using fastembed
- **Real-time TUI** - Interactive terminal dashboard with live charts
- **Headless Mode** - JSON/CSV output for CI pipelines and automation
- **Detailed Metrics** - p50, p90, p99 latencies, QPS, success/failure rates

## Quick Start

```bash
# Install (requires Rust)
cargo install qstorm-cli --features all-providers,embeddings

# Create config and queries
qstorm -c qstorm.yaml -q queries.yaml
```

## Example Output

```
┌─ qstorm [my-qdrant] (10 queries) - RUNNING ─────────────────┐
│                                                              │
│  ┌─ Queries/Second ─┐  ┌─ Latency p50 (ms) ─┐               │
│  │     ▄▄▄▄▄▄▄▄     │  │  ▂▃▄▅▆▅▄▃▂▁       │               │
│  │  ▄▄█████████▄    │  │                     │               │
│  └──────────────────┘  └─────────────────────┘               │
│                                                              │
│  QPS: 156.2 | p50: 12.4ms | p99: 45.2ms | Success: 100      │
└──────────────────────────────────────────────────────────────┘
```

## Supported Providers

| Provider | Vector Search | Status |
|----------|--------------|--------|
| Qdrant | Yes | Supported |
| Elasticsearch | Yes | Supported |
| OpenSearch | - | Planned |
| Weaviate | - | Planned |

## License

MIT License