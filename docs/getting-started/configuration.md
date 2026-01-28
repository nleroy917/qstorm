# Configuration Reference

qstorm uses YAML configuration files. This page documents all available options.

## Full Example

```yaml
provider:
  name: "my-elasticsearch"
  type: elasticsearch
  url: "http://localhost:9200"
  index: "products"
  credentials:
    type: basic
    username: "elastic"
    password: "changeme"
  vector_field: "embedding"

benchmark:
  warmup_iterations: 10
  burst_size: 100
  concurrency: 10
  timeout_ms: 5000
  top_k: 10

embedding:
  model: "BAAI/bge-small-en-v1.5"
```

## Provider Settings

### `provider.name`
**Required** - Display name for this provider instance.

```yaml
provider:
  name: "production-qdrant"
```

### `provider.type`
**Required** - Provider type. One of: `elasticsearch`, `qdrant`

```yaml
provider:
  type: qdrant
```

### `provider.url`
**Required** - Connection URL for the provider.

```yaml
provider:
  url: "http://localhost:6333"
```

### `provider.index`
**Required** - Index name (Elasticsearch) or collection name (Qdrant).

```yaml
provider:
  index: "my-collection"
```

### `provider.vector_field`
Name of the field containing vectors. Default: `"vector"`

```yaml
provider:
  vector_field: "embedding"
```

### `provider.credentials`
Authentication credentials. Optional.

**Basic Auth:**
```yaml
provider:
  credentials:
    type: basic
    username: "user"
    password: "pass"
```

**API Key:**
```yaml
provider:
  credentials:
    type: api_key
    key: "your-api-key"
```

**Bearer Token:**
```yaml
provider:
  credentials:
    type: bearer
    token: "your-token"
```

## Benchmark Settings

### `benchmark.warmup_iterations`
Number of queries to run before measuring. Default: `10`

```yaml
benchmark:
  warmup_iterations: 50
```

### `benchmark.burst_size`
Number of queries per burst. Default: `100`

```yaml
benchmark:
  burst_size: 200
```

### `benchmark.concurrency`
Maximum concurrent requests within a burst. Default: `10`

```yaml
benchmark:
  concurrency: 20
```

### `benchmark.timeout_ms`
Request timeout in milliseconds. Default: `5000`

```yaml
benchmark:
  timeout_ms: 10000
```

### `benchmark.top_k`
Number of results to request per query. Default: `10`

```yaml
benchmark:
  top_k: 50
```

## Embedding Settings

### `embedding.model`
Embedding model for converting text queries to vectors. Default: `"BAAI/bge-small-en-v1.5"`

Supported models:

| Model | Dimensions | Notes |
|-------|-----------|-------|
| `BAAI/bge-small-en-v1.5` | 384 | Fast, good quality |
| `BAAI/bge-base-en-v1.5` | 768 | Better quality, slower |
| `BAAI/bge-large-en-v1.5` | 1024 | Best quality, slowest |
| `all-MiniLM-L6-v2` | 384 | Lightweight alternative |
| `all-MiniLM-L12-v2` | 384 | Slightly better than L6 |

```yaml
embedding:
  model: "BAAI/bge-base-en-v1.5"
```

!!! warning "Model Dimensions"
    Ensure your embedding model dimensions match the vectors in your index. Using mismatched dimensions will cause search errors.