use anyhow::{Result, anyhow};
use qstorm_core::{
    BurstMetrics, Config, EmbeddedQuery, Embedder, QueryFile,
    config::{ProviderConfig, ProviderType},
    runner::BenchmarkRunner,
};

/// Application state
pub struct App {
    pub config: Config,
    runner: Option<BenchmarkRunner>,
    queries: Vec<EmbeddedQuery>,
    pub state: AppState,
    pub history: MetricsHistory,
    pub status_message: Option<String>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Idle,
    Connecting,
    Warming,
    Running,
    Paused,
    Error,
}

/// Rolling history of metrics for charting
pub struct MetricsHistory {
    pub bursts: Vec<BurstMetrics>,
    pub max_history: usize,
}

impl Default for MetricsHistory {
    fn default() -> Self {
        Self {
            bursts: Vec::new(),
            max_history: 100,
        }
    }
}

impl MetricsHistory {
    pub fn push(&mut self, metrics: BurstMetrics) {
        self.bursts.push(metrics);
        if self.bursts.len() > self.max_history {
            self.bursts.remove(0);
        }
    }

    pub fn latest(&self) -> Option<&BurstMetrics> {
        self.bursts.last()
    }

    pub fn qps_series(&self) -> Vec<(f64, f64)> {
        self.bursts
            .iter()
            .enumerate()
            .map(|(i, m)| (i as f64, m.qps))
            .collect()
    }

    pub fn p50_series(&self) -> Vec<(f64, f64)> {
        self.bursts
            .iter()
            .enumerate()
            .map(|(i, m)| (i as f64, m.latency.p50_us as f64 / 1000.0))
            .collect()
    }

    pub fn p99_series(&self) -> Vec<(f64, f64)> {
        self.bursts
            .iter()
            .enumerate()
            .map(|(i, m)| (i as f64, m.latency.p99_us as f64 / 1000.0))
            .collect()
    }

    pub fn recall_series(&self) -> Vec<(f64, f64)> {
        self.bursts
            .iter()
            .enumerate()
            .filter_map(|(i, m)| m.recall_at_k.map(|r| (i as f64, r * 100.0)))
            .collect()
    }
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            runner: None,
            queries: Vec::new(),
            state: AppState::Idle,
            history: MetricsHistory::default(),
            status_message: None,
        })
    }

    pub fn provider_name(&self) -> &str {
        &self.config.provider.name
    }

    pub fn query_count(&self) -> usize {
        self.queries.len()
    }

    /// Load queries from file and embed them
    pub fn load_and_embed_queries(&mut self, query_file_path: &str) -> Result<()> {
        self.status_message = Some("Loading queries...".into());

        let query_file = QueryFile::from_file(query_file_path)?;
        if query_file.queries.is_empty() {
            return Err(anyhow!("Query file contains no queries"));
        }

        self.status_message = Some(format!("Embedding {} queries...", query_file.queries.len()));

        let model_name = self
            .config
            .embedding
            .as_ref()
            .map(|e| e.model.as_str())
            .unwrap_or("BAAI/bge-small-en-v1.5");

        let embedder = Embedder::new(model_name)?;
        self.queries = embedder.embed_queries(&query_file.queries)?;

        self.status_message = Some(format!("Loaded {} queries", self.queries.len()));
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.state = AppState::Connecting;
        self.status_message = Some("Connecting to provider...".into());

        let provider = create_provider(&self.config.provider)?;
        let runner = BenchmarkRunner::new(provider, self.config.benchmark.clone())
            .with_queries(self.queries.clone());

        let mut runner = runner;
        runner.connect().await?;
        self.runner = Some(runner);
        self.state = AppState::Idle;
        self.status_message = Some("Connected".into());
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(runner) = &mut self.runner {
            runner.disconnect().await?;
        }
        self.runner = None;
        self.state = AppState::Idle;
        self.status_message = Some("Disconnected".into());
        Ok(())
    }

    pub async fn warmup(&mut self) -> Result<()> {
        self.state = AppState::Warming;
        self.status_message = Some("Warming up...".into());

        if let Some(runner) = &mut self.runner {
            runner.warmup().await?;
        }

        self.state = AppState::Idle;
        self.status_message = Some("Warmup complete".into());
        Ok(())
    }

    pub async fn run_burst(&mut self) -> Result<BurstMetrics> {
        self.state = AppState::Running;

        let runner = self
            .runner
            .as_mut()
            .ok_or_else(|| anyhow!("Not connected"))?;

        let metrics = runner.run_burst().await?;
        self.history.push(metrics.clone());
        self.state = AppState::Idle;
        Ok(metrics)
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            AppState::Running | AppState::Idle => AppState::Paused,
            AppState::Paused => AppState::Idle,
            _ => self.state,
        };
    }
}

fn create_provider(config: &ProviderConfig) -> Result<Box<dyn qstorm_core::SearchProvider>> {
    match config.provider_type {
        #[cfg(feature = "elasticsearch")]
        ProviderType::Elasticsearch => Ok(Box::new(
            qstorm_core::providers::ElasticsearchProvider::new(config.clone()),
        )),

        #[cfg(feature = "qdrant")]
        ProviderType::Qdrant => Ok(Box::new(qstorm_core::providers::QdrantProvider::new(
            config.clone(),
        ))),

        #[allow(unreachable_patterns)]
        _ => Err(anyhow!(
            "Provider {:?} is not enabled in this build",
            config.provider_type
        )),
    }
}
