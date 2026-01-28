# Quickstart

This guide walks you through running your first benchmark with qstorm.

## Prerequisites

- A running vector search engine (Qdrant or Elasticsearch)
- An index/collection with vector data
- qstorm installed with embeddings support

## Step 1: Create Configuration

Create `qstorm.yaml` in your working directory:

```yaml
provider:
  name: "my-qdrant"
  type: qdrant
  url: "http://localhost:6333"
  index: "my-collection"
  vector_field: "vector"

benchmark:
  warmup_iterations: 10
  burst_size: 100
  concurrency: 10
  timeout_ms: 5000
  top_k: 10

embedding:
  model: "BAAI/bge-small-en-v1.5"
```

## Step 2: Create Query File

Create `queries.yaml` with text queries to benchmark:

```yaml
queries:
  - "wireless bluetooth headphones"
  - "ergonomic office chair"
  - "mechanical keyboard"
  - "ultrawide monitor"
  - "standing desk"
```

## Step 3: Run the Benchmark

### Interactive TUI Mode

```bash
qstorm -c qstorm.yaml -q queries.yaml
```

This launches the interactive terminal UI with live charts.

**Controls:**
- `Space` - Pause/Resume
- `q` or `Esc` - Quit

### Headless Mode

For scripting or CI:

```bash
# Run 10 bursts, output as CSV
qstorm -c qstorm.yaml -q queries.yaml --headless --bursts 10 --output csv
```

Output:
```csv
timestamp,qps,p50_ms,p90_ms,p99_ms,success,failure
2025-01-27T10:30:00Z,156.23,12.45,28.90,45.23,100,0
2025-01-27T10:30:01Z,162.45,11.89,27.12,42.56,100,0
...
```

## What Happens

1. **Load queries** - Reads your query file
2. **Embed queries** - Converts text to vectors using the configured model (first run downloads ~50MB model)
3. **Connect** - Establishes connection to your vector database
4. **Warmup** - Runs warmup queries to stabilize performance
5. **Benchmark** - Executes bursts and collects metrics

## Next Steps

- [Configuration Reference](configuration.md) - All configuration options
- [CLI Reference](../usage/cli.md) - Command-line options
- [Provider Setup](../providers/index.md) - Provider-specific guides