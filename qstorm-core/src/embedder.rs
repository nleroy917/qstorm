#[cfg(feature = "embeddings")]
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

use crate::error::{Error, Result};
use crate::queries::EmbeddedQuery;

/// Wrapper around fastembed for generating query embeddings
pub struct Embedder {
    #[cfg(feature = "embeddings")]
    model: TextEmbedding,
    #[cfg(not(feature = "embeddings"))]
    _phantom: std::marker::PhantomData<()>,
}

impl Embedder {
    /// Create a new embedder with the specified model
    #[cfg(feature = "embeddings")]
    pub fn new(model_name: &str) -> Result<Self> {
        let model = parse_model(model_name)?;
        let embedding =
            TextEmbedding::try_new(InitOptions::new(model).with_show_download_progress(true))
                .map_err(|e| Error::Config(format!("Failed to load embedding model: {}", e)))?;

        Ok(Self { model: embedding })
    }

    #[cfg(not(feature = "embeddings"))]
    pub fn new(_model_name: &str) -> Result<Self> {
        Err(Error::Config(
            "Embeddings feature not enabled. Rebuild with --features embeddings".into(),
        ))
    }

    /// Embed a batch of text queries
    #[cfg(feature = "embeddings")]
    pub fn embed_queries(&self, texts: &[String]) -> Result<Vec<EmbeddedQuery>> {
        let embeddings = self
            .model
            .embed(texts.to_vec(), None)
            .map_err(|e| Error::Config(format!("Embedding failed: {}", e)))?;

        let queries = texts
            .iter()
            .zip(embeddings.into_iter())
            .map(|(text, vector)| EmbeddedQuery {
                text: text.clone(),
                vector,
            })
            .collect();

        Ok(queries)
    }

    #[cfg(not(feature = "embeddings"))]
    pub fn embed_queries(&self, _texts: &[String]) -> Result<Vec<EmbeddedQuery>> {
        Err(Error::Config("Embeddings feature not enabled".into()))
    }

    /// Get the embedding dimension for this model
    #[cfg(feature = "embeddings")]
    pub fn dimension(&self) -> usize {
        // fastembed doesn't expose this directly, so we embed a test string
        self.model
            .embed(vec!["test"], None)
            .map(|v| v.first().map(|e| e.len()).unwrap_or(0))
            .unwrap_or(0)
    }

    #[cfg(not(feature = "embeddings"))]
    pub fn dimension(&self) -> usize {
        0
    }
}

#[cfg(feature = "embeddings")]
fn parse_model(name: &str) -> Result<EmbeddingModel> {
    // Map common model names to fastembed enum variants
    let model = match name {
        "BAAI/bge-small-en-v1.5" | "bge-small-en-v1.5" => EmbeddingModel::BGESmallENV15,
        "BAAI/bge-base-en-v1.5" | "bge-base-en-v1.5" => EmbeddingModel::BGEBaseENV15,
        "BAAI/bge-large-en-v1.5" | "bge-large-en-v1.5" => EmbeddingModel::BGELargeENV15,
        "sentence-transformers/all-MiniLM-L6-v2" | "all-MiniLM-L6-v2" => {
            EmbeddingModel::AllMiniLML6V2
        }
        "sentence-transformers/all-MiniLM-L12-v2" | "all-MiniLM-L12-v2" => {
            EmbeddingModel::AllMiniLML12V2
        }
        _ => {
            return Err(Error::Config(format!(
                "Unknown embedding model: {}. Supported: bge-small-en-v1.5, bge-base-en-v1.5, bge-large-en-v1.5, all-MiniLM-L6-v2, all-MiniLM-L12-v2",
                name
            )));
        }
    };
    Ok(model)
}
