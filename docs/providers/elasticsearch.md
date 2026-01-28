# Elasticsearch

[Elasticsearch](https://www.elastic.co/elasticsearch/) supports vector search via the kNN feature (8.0+).

## Configuration

```yaml
provider:
  name: "my-elasticsearch"
  type: elasticsearch
  url: "http://localhost:9200"
  index: "my-index"
  vector_field: "embedding"
```

## Authentication

### No Authentication (Local)

```yaml
provider:
  name: "local-es"
  type: elasticsearch
  url: "http://localhost:9200"
  index: "my-index"
```

### Basic Auth

```yaml
provider:
  name: "es-basic"
  type: elasticsearch
  url: "https://localhost:9200"
  index: "my-index"
  credentials:
    type: basic
    username: "elastic"
    password: "your-password"
```

### API Key

```yaml
provider:
  name: "es-apikey"
  type: elasticsearch
  url: "https://localhost:9200"
  index: "my-index"
  credentials:
    type: api_key
    key: "your-api-key"
```

### Elastic Cloud

```yaml
provider:
  name: "elastic-cloud"
  type: elasticsearch
  url: "https://my-deployment.es.us-east-1.aws.cloud.es.io:9243"
  index: "my-index"
  credentials:
    type: api_key
    key: "your-cloud-api-key"
```

## Index Requirements

Your Elasticsearch index must have a `dense_vector` field:

```json
{
  "mappings": {
    "properties": {
      "embedding": {
        "type": "dense_vector",
        "dims": 384,
        "index": true,
        "similarity": "cosine"
      },
      "text": {
        "type": "text"
      }
    }
  }
}
```

### Important Settings

- `index: true` - Required for kNN search
- `similarity` - Match your embedding model (usually `cosine`)
- `dims` - Must match embedding dimensions

## Example Setup

### 1. Start Elasticsearch

```bash
docker run -p 9200:9200 -e "discovery.type=single-node" \
  -e "xpack.security.enabled=false" \
  elasticsearch:8.12.0
```

### 2. Create Index

```bash
curl -X PUT http://localhost:9200/test \
  -H 'Content-Type: application/json' \
  -d '{
    "mappings": {
      "properties": {
        "embedding": {
          "type": "dense_vector",
          "dims": 384,
          "index": true,
          "similarity": "cosine"
        }
      }
    }
  }'
```

### 3. Index Documents

```bash
curl -X POST http://localhost:9200/test/_bulk \
  -H 'Content-Type: application/x-ndjson' \
  -d '
{"index": {"_id": "1"}}
{"embedding": [0.1, 0.2, ...], "text": "example document"}
'
```

### 4. Run Benchmark

```bash
qstorm -c qstorm.yaml -q queries.yaml
```

## How qstorm Queries Elasticsearch

qstorm uses the kNN query:

```json
{
  "size": 10,
  "knn": {
    "field": "embedding",
    "query_vector": [0.1, 0.2, ...],
    "k": 10,
    "num_candidates": 100
  }
}
```

The `num_candidates` is set to `top_k * 10` for a balance of speed and accuracy.

## Troubleshooting

### "index_not_found_exception"

Ensure the index exists:

```bash
curl http://localhost:9200/my-index
```

### "illegal_argument_exception" for kNN

Ensure your field is:
1. Type `dense_vector`
2. Has `index: true`
3. Dimensions match your query vectors

### SSL Certificate Errors

For self-signed certificates, you may need to configure the Elasticsearch client to accept them. Currently qstorm uses the default certificate validation.

## Performance Tips

- Use `index: true` with HNSW (default in ES 8.x)
- Increase `num_candidates` for better recall (slower)
- Use dedicated ML nodes for vector operations
- Consider quantization for large indices

## Version Compatibility

| Elasticsearch | Status |
|--------------|--------|
| 8.x | Supported |
| 7.x | Not tested (kNN syntax differs) |
| < 7.x | Not supported |