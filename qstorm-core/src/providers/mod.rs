#[cfg(feature = "elasticsearch")]
pub mod elastic;

#[cfg(feature = "qdrant")]
pub mod qdrant;

// re-export provider types when features are enabled
#[cfg(feature = "elasticsearch")]
pub use elastic::ElasticsearchProvider;

#[cfg(feature = "qdrant")]
pub use qdrant::QdrantProvider;