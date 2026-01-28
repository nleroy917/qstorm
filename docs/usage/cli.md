# CLI Reference

## Synopsis

```bash
qstorm [OPTIONS] --queries <QUERIES>
```

## Options

### `-c, --config <CONFIG>`
Path to configuration file. Default: `qstorm.yaml`

```bash
qstorm -c /path/to/config.yaml -q queries.yaml
```

### `-q, --queries <QUERIES>`
**Required** - Path to queries file (YAML format).

```bash
qstorm -q ./my-queries.yaml
```

### `--headless`
Run without the TUI, output results to stdout.

```bash
qstorm -q queries.yaml --headless
```

### `-b, --bursts <BURSTS>`
Number of bursts to run. Default: `0` (continuous until stopped).

```bash
# Run exactly 10 bursts then exit
qstorm -q queries.yaml --headless --bursts 10
```

### `--output <OUTPUT>`
Output format for headless mode. Options: `json`, `csv`. Default: `json`

```bash
qstorm -q queries.yaml --headless --output csv
```

### `-h, --help`
Print help information.

## Examples

### Interactive Benchmarking

```bash
qstorm -c qstorm.yaml -q queries.yaml
```

### CI Pipeline

```bash
# Run 100 bursts, output CSV for analysis
qstorm -c qstorm.yaml -q queries.yaml \
  --headless \
  --bursts 100 \
  --output csv > results.csv
```

### Quick Test

```bash
# Run 5 bursts to verify setup
qstorm -c qstorm.yaml -q queries.yaml --headless --bursts 5
```

## Output Formats

### JSON (default)

Each line is a complete burst result:

```json
{
  "timestamp": "2025-01-27T10:30:00Z",
  "duration_ms": 645,
  "query_count": 100,
  "success_count": 100,
  "failure_count": 0,
  "latency": {
    "min_us": 8234,
    "max_us": 89234,
    "mean_us": 15234.5,
    "p50_us": 12450,
    "p90_us": 28900,
    "p95_us": 35600,
    "p99_us": 45230
  },
  "qps": 155.04
}
```

### CSV

Header followed by one row per burst:

```csv
timestamp,qps,p50_ms,p90_ms,p99_ms,success,failure
2025-01-27T10:30:00Z,155.04,12.45,28.90,45.23,100,0
2025-01-27T10:30:01Z,162.34,11.89,27.12,42.56,100,0
```

## Environment Variables

### `RUST_LOG`
Control logging verbosity:

```bash
# Debug logging
RUST_LOG=debug qstorm -q queries.yaml

# Only warnings and errors
RUST_LOG=warn qstorm -q queries.yaml
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Configuration error |
| 2 | Connection failed |
| 3 | Query file not found |