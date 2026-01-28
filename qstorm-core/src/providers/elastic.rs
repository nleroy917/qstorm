use async_trait::async_trait;
use elasticsearch::{
    auth::Credentials as EsCredentials,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch, SearchParts,
};
use serde_json::json;
use tracing::debug;

use crate::config::{Credentials, ProviderConfig};
use crate::error::{Error, Result};
use crate::provider::{Capabilities, SearchProvider};
use crate::types::{SearchParams, SearchResult, SearchResults};

pub struct ElasticsearchProvider {
    config: ProviderConfig,
    client: Option<Elasticsearch>,
}

impl ElasticsearchProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            config,
            client: None,
        }
    }

    fn client(&self) -> Result<&Elasticsearch> {
        self.client.as_ref().ok_or(Error::NotConnected)
    }
}

#[async_trait]
impl SearchProvider for ElasticsearchProvider {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            vector_search: true,
            vector_dimension: None,
        }
    }

    async fn connect(&mut self) -> Result<()> {
        let url = self
            .config
            .url
            .parse()
            .map_err(|e| Error::Config(format!("Invalid URL: {}", e)))?;

        let pool = SingleNodeConnectionPool::new(url);
        let mut builder = TransportBuilder::new(pool);

        if let Some(creds) = &self.config.credentials {
            builder = match creds {
                Credentials::Basic { username, password } => {
                    builder.auth(EsCredentials::Basic(username.clone(), password.clone()))
                }
                Credentials::ApiKey { key } => builder.auth(EsCredentials::ApiKey(
                    key.clone(),
                    "".to_string(),
                )),
                Credentials::Bearer { token } => {
                    builder.auth(EsCredentials::Bearer(token.clone()))
                }
            };
        }

        let transport = builder
            .build()
            .map_err(|e| Error::Connection(e.to_string()))?;

        let client = Elasticsearch::new(transport);

        // Verify connection
        let response = client
            .cat()
            .health()
            .send()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        if !response.status_code().is_success() {
            return Err(Error::Connection("Health check failed".into()));
        }

        debug!(index = %self.config.index, "Connected to Elasticsearch");
        self.client = Some(client);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.client = None;
        Ok(())
    }

    async fn health_check(&self) -> Result<bool> {
        let client = self.client()?;
        let response = client
            .cat()
            .health()
            .send()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(response.status_code().is_success())
    }

    async fn vector_search(&self, vector: &[f32], params: &SearchParams) -> Result<SearchResults> {
        let client = self.client()?;
        let vector_field = self.config.vector_field.as_deref().unwrap_or("vector");

        let body = json!({
            "size": params.top_k,
            "knn": {
                "field": vector_field,
                "query_vector": vector,
                "k": params.top_k,
                "num_candidates": params.top_k * 10
            }
        });

        let response = client
            .search(SearchParts::Index(&[&self.config.index]))
            .body(body)
            .send()
            .await
            .map_err(|e| Error::QueryExecution(e.to_string()))?;

        if !response.status_code().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(Error::QueryExecution(format!(
                "Search failed: {}",
                error_body
            )));
        }

        let response_body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        let took_ms = response_body["took"].as_u64();
        let total_hits = response_body["hits"]["total"]["value"].as_u64();

        let hits = response_body["hits"]["hits"]
            .as_array()
            .ok_or_else(|| Error::InvalidResponse("Missing hits array".into()))?;

        let results: Vec<SearchResult> = hits
            .iter()
            .filter_map(|hit| {
                let id = hit["_id"].as_str()?.to_string();
                let score = hit["_score"].as_f64().unwrap_or(0.0) as f32;
                let payload = if params.include_payload {
                    hit.get("_source").cloned()
                } else {
                    None
                };
                Some(SearchResult { id, score, payload })
            })
            .collect();

        let mut search_results = SearchResults::new(results);
        if let Some(took) = took_ms {
            search_results = search_results.with_took(took);
        }
        if let Some(total) = total_hits {
            search_results = search_results.with_total_hits(total);
        }

        Ok(search_results)
    }
}