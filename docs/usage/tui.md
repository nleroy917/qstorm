# Terminal UI

qstorm includes an interactive terminal dashboard for real-time monitoring.

## Starting the TUI

```bash
qstorm -c qstorm.yaml -q queries.yaml
```

## Layout

```
┌─ qstorm [provider-name] (N queries) - STATUS ───────────────┐
│                                                              │
│  ┌─ Queries/Second ─────┐  ┌─ Latency p50 (ms) ──────┐     │
│  │                      │  │                          │     │
│  │  [Live QPS Chart]    │  │  [Live p50 Chart]        │     │
│  │                      │  │                          │     │
│  └──────────────────────┘  └──────────────────────────┘     │
│                                                              │
│  ┌─ Latency p99 (ms) ───┐  ┌─ Recall@k (%) ──────────┐     │
│  │                      │  │                          │     │
│  │  [Live p99 Chart]    │  │  [Recall Chart or N/A]   │     │
│  │                      │  │                          │     │
│  └──────────────────────┘  └──────────────────────────┘     │
│                                                              │
│  QPS: 156.2 | p50: 12.4ms | p99: 45.2ms | Success: 100     │
└──────────────────────────────────────────────────────────────┘
```

## Header

Shows:
- **Provider name** - From your config
- **Query count** - Number of embedded queries
- **Status** - Current state (IDLE, RUNNING, PAUSED, ERROR)

## Charts

### Queries/Second (QPS)
Throughput over time. Higher is better.

### Latency p50 (ms)
Median response time. Shows typical user experience.

### Latency p99 (ms)
99th percentile response time. Shows worst-case latency.

### Recall@k (%)
Search quality metric. Only shown when ground truth is provided.
Currently displays "no ground truth" placeholder.

## Footer

Real-time metrics from the most recent burst:
- **QPS** - Queries per second achieved
- **p50** - Median latency in milliseconds
- **p99** - 99th percentile latency in milliseconds
- **Success** - Successful queries in last burst
- **Failed** - Failed queries in last burst

## Keyboard Controls

| Key | Action |
|-----|--------|
| `Space` | Pause/Resume benchmarking |
| `q` | Quit |
| `Esc` | Quit |

## States

| State | Description |
|-------|-------------|
| **IDLE** | Ready, waiting between bursts |
| **CONNECTING** | Establishing connection to provider |
| **WARMING** | Running warmup queries |
| **RUNNING** | Executing a burst |
| **PAUSED** | Benchmark paused by user |
| **ERROR** | An error occurred |

## Burst Timing

By default, bursts run every 1 second. The timing is:

1. Execute burst (variable time based on queries)
2. Wait until 1 second has elapsed since burst start
3. Start next burst

This provides consistent data points for the charts.

## Terminal Requirements

- Minimum size: 80x24 characters
- Supports: Most modern terminals (iTerm2, Terminal.app, Windows Terminal, etc.)
- Unicode support required for chart rendering

## Troubleshooting

### Charts not rendering
Ensure your terminal supports Unicode box-drawing characters and has sufficient size.

### Display corruption on exit
If the terminal doesn't restore properly after quitting:
```bash
reset
```

### Colors not showing
Check that your terminal supports 256 colors or true color.