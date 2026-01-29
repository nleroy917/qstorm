use async_trait::async_trait;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{PointId, SearchPointsBuilder};
use tracing::debug;

use crate::config::{Credentials, ProviderConfig};
use crate::error::{Error, Result};
use crate::provider::{Capabilities, SearchProvider};
use crate::types::{SearchParams, SearchResult, SearchResults};

pub struct QdrantProvider {
    config: ProviderConfig,
    client: Option<Qdrant>,
}

impl QdrantProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            config,
            client: None,
        }
    }

    fn client(&self) -> Result<&Qdrant> {
        self.client.as_ref().ok_or(Error::NotConnected)
    }
}

#[async_trait]
impl SearchProvider for QdrantProvider {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            vector_search: true,
            vector_dimension: None, // Could query collection info to get this
        }
    }

    async fn connect(&mut self) -> Result<()> {
        let mut builder = Qdrant::from_url(&self.config.url);

        if let Some(creds) = &self.config.credentials {
            builder = match creds {
                Credentials::ApiKey { key } => builder.api_key(key.clone()),
                _ => {
                    return Err(Error::Config(
                        "Qdrant only supports API key authentication".into(),
                    ));
                }
            };
        }

        let client = builder
            .build()
            .map_err(|e| Error::Connection(e.to_string()))?;

        // Verify connection by checking collection exists
        let collections = client
            .list_collections()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        let exists = collections
            .collections
            .iter()
            .any(|c| c.name == self.config.index);

        if !exists {
            return Err(Error::Config(format!(
                "Collection '{}' not found",
                self.config.index
            )));
        }

        debug!(collection = %self.config.index, "Connected to Qdrant");
        self.client = Some(client);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.client = None;
        Ok(())
    }

    async fn health_check(&self) -> Result<bool> {
        let client = self.client()?;
        client
            .health_check()
            .await
            .map(|_| true)
            .map_err(|e| Error::Connection(e.to_string()))
    }

    async fn vector_search(&self, vector: &[f32], params: &SearchParams) -> Result<SearchResults> {
        let client = self.client()?;
        let vector_field = self.config.vector_field.as_deref();

        let mut search =
            SearchPointsBuilder::new(&self.config.index, vector.to_vec(), params.top_k as u64);

        if let Some(field) = vector_field {
            search = search.vector_name(field.to_string());
        }

        if params.include_payload {
            search = search.with_payload(true);
        }

        if let Some(min_score) = params.min_score {
            search = search.score_threshold(min_score);
        }

        let response = client
            .search_points(search)
            .await
            .map_err(|e| Error::QueryExecution(e.to_string()))?;

        let results: Vec<SearchResult> = response
            .result
            .into_iter()
            .map(|point| {
                let id = match point.id {
                    Some(PointId {
                        point_id_options: Some(id),
                    }) => {
                        use qdrant_client::qdrant::point_id::PointIdOptions;
                        match id {
                            PointIdOptions::Num(n) => n.to_string(),
                            PointIdOptions::Uuid(s) => s,
                        }
                    }
                    _ => "unknown".to_string(),
                };

                let payload = if params.include_payload {
                    Some(serde_json::to_value(&point.payload).unwrap_or_default())
                } else {
                    None
                };

                SearchResult {
                    id,
                    score: point.score,
                    payload,
                }
            })
            .collect();

        Ok(SearchResults::new(results))
    }
}
