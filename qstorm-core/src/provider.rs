use async_trait::async_trait;

use crate::error::Result;
use crate::types::{SearchParams, SearchResults};

/// Capabilities advertised by a search provider
#[derive(Debug, Clone, Default)]
pub struct Capabilities {
    pub vector_search: bool,
    pub vector_dimension: Option<usize>,
}

/// Trait for vector search providers
#[async_trait]
pub trait SearchProvider: Send + Sync {
    /// Human-readable name for this provider instance
    fn name(&self) -> &str;

    /// What capabilities this provider supports
    fn capabilities(&self) -> Capabilities;

    /// Establish connection to the search engine
    async fn connect(&mut self) -> Result<()>;

    /// Close connection gracefully
    async fn disconnect(&mut self) -> Result<()>;

    /// Check if the provider is healthy and connected
    async fn health_check(&self) -> Result<bool>;

    /// Execute a vector similarity search
    async fn vector_search(&self, vector: &[f32], params: &SearchParams) -> Result<SearchResults>;
}