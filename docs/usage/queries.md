# Query Files

qstorm uses YAML files to define the queries used during benchmarking.

## Basic Format

```yaml
queries:
  - "first search query"
  - "second search query"
  - "third search query"
```

Each query is a text string that will be embedded into a vector at startup.

## Example: E-commerce

```yaml
queries:
  - "wireless bluetooth headphones with noise cancellation"
  - "ergonomic office chair for back pain"
  - "mechanical keyboard for programming"
  - "ultrawide monitor for productivity"
  - "standing desk converter"
  - "laptop cooling pad"
  - "USB-C hub with multiple ports"
  - "comfortable mouse for long hours"
  - "blue light blocking glasses"
  - "desk organizer for cables"
```

## Example: Documentation Search

```yaml
queries:
  - "how to install the application"
  - "configuration options"
  - "troubleshooting connection errors"
  - "API authentication methods"
  - "rate limiting and quotas"
  - "webhook integration guide"
  - "database migration steps"
  - "performance optimization tips"
```

## How Queries Are Used

1. **At startup** - All queries are loaded and embedded using the configured model
2. **During bursts** - Queries are cycled through round-robin
3. **Concurrently** - Multiple queries may be in-flight simultaneously

### Query Cycling

If you have 10 queries and `burst_size: 100`, each query will be executed approximately 10 times per burst.

```
Burst 1: q1, q2, q3, ..., q10, q1, q2, ..., q10, q1, ...
Burst 2: q1, q2, q3, ..., q10, q1, q2, ..., q10, q1, ...
```

## Best Practices

### Realistic Queries
Use queries that represent actual user searches in your application:

```yaml
# Good - realistic user queries
queries:
  - "comfortable running shoes for flat feet"
  - "waterproof hiking boots size 10"
  - "casual sneakers under $100"

# Bad - synthetic/unrealistic
queries:
  - "test query 1"
  - "aaaaaa"
  - "product"
```

### Query Diversity
Include a variety of query types to stress-test different aspects:

```yaml
queries:
  # Short queries
  - "laptop"
  - "headphones"

  # Long queries
  - "best laptop for software development with 32gb ram"
  - "noise cancelling headphones for open office environment"

  # Specific queries
  - "MacBook Pro 14 inch M3 Pro"

  # Broad queries
  - "electronics accessories"
```

### Query Count
- **Minimum**: 5-10 queries for basic testing
- **Recommended**: 50-100 queries for realistic benchmarks
- **Maximum**: No hard limit, but embedding time increases linearly

## Embedding Considerations

### Model Selection

The embedding model must match what was used to create vectors in your index:

| Your Index | Config |
|------------|--------|
| Created with `bge-small-en-v1.5` | `model: "BAAI/bge-small-en-v1.5"` |
| Created with `text-embedding-ada-002` | Not supported (use pre-computed vectors) |

### First Run

On first run, fastembed downloads the model (~50-150MB depending on model). Subsequent runs use cached models.

```
Loading and embedding queries (this may take a moment)...
Embedded 10 queries. Starting TUI...
```