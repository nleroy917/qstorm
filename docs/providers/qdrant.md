# Qdrant

[Qdrant](https://qdrant.tech) is a vector similarity search engine with a convenient API.

## Configuration

```yaml
provider:
  name: "my-qdrant"
  type: qdrant
  url: "http://localhost:6333"
  index: "my-collection"
  vector_field: "vector"
```

## Authentication

### No Authentication (Local)

```yaml
provider:
  name: "local-qdrant"
  type: qdrant
  url: "http://localhost:6333"
  index: "my-collection"
```

### API Key (Qdrant Cloud)

```yaml
provider:
  name: "qdrant-cloud"
  type: qdrant
  url: "https://xyz-123.us-east-1.aws.cloud.qdrant.io:6333"
  index: "my-collection"
  credentials:
    type: api_key
    key: "your-api-key-here"
```

## Vector Field

Qdrant supports named vectors. Specify the vector name:

```yaml
provider:
  vector_field: "text_embedding"  # Default: "vector"
```

For collections with unnamed vectors (single vector per point), use the default `"vector"`.

## Collection Requirements

Your Qdrant collection must:

1. Exist before running qstorm
2. Have vectors indexed with matching dimensions
3. Be accessible from the qstorm host

### Verify Collection

```bash
curl http://localhost:6333/collections/my-collection
```

### Expected Response

```json
{
  "result": {
    "status": "green",
    "vectors_count": 10000,
    "points_count": 10000,
    "config": {
      "params": {
        "vectors": {
          "size": 384,
          "distance": "Cosine"
        }
      }
    }
  }
}
```

## Example Setup

### 1. Start Qdrant

```bash
docker run -p 6333:6333 qdrant/qdrant
```

### 2. Create Collection

```bash
curl -X PUT http://localhost:6333/collections/test \
  -H 'Content-Type: application/json' \
  -d '{
    "vectors": {
      "size": 384,
      "distance": "Cosine"
    }
  }'
```

### 3. Insert Test Data

```bash
curl -X PUT http://localhost:6333/collections/test/points \
  -H 'Content-Type: application/json' \
  -d '{
    "points": [
      {"id": 1, "vector": [0.1, 0.2, ...], "payload": {"text": "example"}},
      ...
    ]
  }'
```

### 4. Run Benchmark

```bash
qstorm -c qstorm.yaml -q queries.yaml
```

## Troubleshooting

### "Collection not found"

Ensure the collection exists and the name matches exactly:

```yaml
provider:
  index: "my-collection"  # Case-sensitive
```

### Connection Refused

Check Qdrant is running and accessible:

```bash
curl http://localhost:6333/health
```

### Dimension Mismatch

Ensure your embedding model produces vectors matching your collection:

- `bge-small-en-v1.5` → 384 dimensions
- `bge-base-en-v1.5` → 768 dimensions
- `bge-large-en-v1.5` → 1024 dimensions

## Performance Tips

- Use HNSW index for best search performance
- Ensure collection is not in "indexing" state during benchmarks
- Consider increasing `ef` parameter for more accurate (but slower) search