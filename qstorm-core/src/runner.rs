use std::sync::Arc;
use std::time::Instant;

use tokio::sync::Semaphore;
use tracing::{debug, info, warn};

use crate::config::BenchmarkConfig;
use crate::error::Result;
use crate::metrics::{BurstMetrics, Metrics};
use crate::provider::SearchProvider;
use crate::queries::EmbeddedQuery;
use crate::types::SearchParams;

/// Orchestrates benchmark execution for vector search
pub struct BenchmarkRunner {
    provider: Box<dyn SearchProvider>,
    config: BenchmarkConfig,
    metrics: Metrics,
    queries: Vec<EmbeddedQuery>,
}

impl BenchmarkRunner {
    pub fn new(provider: Box<dyn SearchProvider>, config: BenchmarkConfig) -> Self {
        Self {
            provider,
            config,
            metrics: Metrics::new(),
            queries: Vec::new(),
        }
    }

    /// Set the embedded queries to use for benchmarking
    pub fn with_queries(mut self, queries: Vec<EmbeddedQuery>) -> Self {
        self.queries = queries;
        self
    }

    /// Get the number of loaded queries
    pub fn query_count(&self) -> usize {
        self.queries.len()
    }

    /// Connect to the provider
    pub async fn connect(&mut self) -> Result<()> {
        self.provider.connect().await
    }

    /// Disconnect from the provider
    pub async fn disconnect(&mut self) -> Result<()> {
        self.provider.disconnect().await
    }

    /// Run warmup iterations (results discarded)
    pub async fn warmup(&mut self) -> Result<()> {
        if self.queries.is_empty() {
            warn!("No queries configured for warmup");
            return Ok(());
        }

        info!(
            iterations = self.config.warmup_iterations,
            "Starting warmup"
        );

        let params = SearchParams {
            top_k: self.config.top_k,
            timeout_ms: self.config.timeout_ms,
            ..Default::default()
        };

        for i in 0..self.config.warmup_iterations {
            let query = &self.queries[i % self.queries.len()];
            let _ = self.provider.vector_search(&query.vector, &params).await;
        }

        info!("Warmup complete");
        Ok(())
    }

    /// Execute a single burst of vector queries
    pub async fn run_burst(&mut self) -> Result<BurstMetrics> {
        if self.queries.is_empty() {
            return Err(crate::error::Error::Config("No queries configured".into()));
        }

        let semaphore = Arc::new(Semaphore::new(self.config.concurrency));
        let params = Arc::new(SearchParams {
            top_k: self.config.top_k,
            timeout_ms: self.config.timeout_ms,
            ..Default::default()
        });

        self.metrics.start_burst();

        // Cycle through queries for this burst
        let query_indices: Vec<usize> = (0..self.config.burst_size)
            .map(|i| i % self.queries.len())
            .collect();

        for idx in query_indices {
            let _permit = semaphore.clone().acquire_owned().await.unwrap();
            let query = &self.queries[idx];
            let params = params.clone();

            let start = Instant::now();
            let result = self.provider.vector_search(&query.vector, &params).await;
            let latency = start.elapsed();

            match result {
                Ok(results) => {
                    // TODO: Calculate recall if ground truth is provided
                    self.metrics.record_success(latency, None);
                    debug!(
                        latency_ms = latency.as_millis(),
                        hits = results.results.len(),
                        query = %query.text,
                        "Query succeeded"
                    );
                }
                Err(e) => {
                    self.metrics.record_failure(latency);
                    warn!(error = %e, latency_ms = latency.as_millis(), "Query failed");
                }
            }
        }

        self.metrics
            .finish_burst()
            .ok_or_else(|| crate::error::Error::Config("No burst in progress".into()))
    }

    /// Get reference to collected metrics
    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    /// Get provider name
    pub fn provider_name(&self) -> &str {
        self.provider.name()
    }
}