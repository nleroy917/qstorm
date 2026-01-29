use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Top-level configuration for qstorm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Provider configuration
    pub provider: ProviderConfig,
    /// Benchmark settings
    #[serde(default)]
    pub benchmark: BenchmarkConfig,
    /// Embedding settings (for semantic/vector queries)
    #[serde(default)]
    pub embedding: Option<EmbeddingConfig>,
    /// Path to query dataset file
    pub queries: Option<String>,
}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn from_str(yaml: &str) -> Result<Self> {
        let config: Config = serde_yaml::from_str(yaml)?;
        Ok(config)
    }
}

/// Configuration for a search provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Display name for this provider
    pub name: String,
    /// Provider type
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    /// Connection URL
    pub url: String,
    /// Index/collection name
    pub index: String,
    /// Authentication credentials
    #[serde(default)]
    pub credentials: Option<Credentials>,
    /// Vector field name (for vector search)
    pub vector_field: Option<String>,
    /// Text field name (for keyword search)
    pub text_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    Elasticsearch,
    Qdrant,
    // in the future: OpenSearch, Weaviate, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Credentials {
    Basic { username: String, password: String },
    ApiKey { key: String },
    Bearer { token: String },
}

/// Benchmark execution settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Number of warmup iterations before measuring
    #[serde(default = "default_warmup")]
    pub warmup_iterations: usize,
    /// Number of queries per burst
    #[serde(default = "default_burst_size")]
    pub burst_size: usize,
    /// Max concurrent requests within a burst
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
    /// Request timeout in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
    /// Top-k for searches
    #[serde(default = "default_top_k")]
    pub top_k: usize,
}

fn default_warmup() -> usize {
    10
}
fn default_burst_size() -> usize {
    100
}
fn default_concurrency() -> usize {
    10
}
fn default_timeout() -> u64 {
    5000
}
fn default_top_k() -> usize {
    10
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: default_warmup(),
            burst_size: default_burst_size(),
            concurrency: default_concurrency(),
            timeout_ms: default_timeout(),
            top_k: default_top_k(),
        }
    }
}

/// Embedding model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Model identifier for fastembed
    #[serde(default = "default_model")]
    pub model: String,
}

fn default_model() -> String {
    "BAAI/bge-small-en-v1.5".to_string()
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
        }
    }
}
