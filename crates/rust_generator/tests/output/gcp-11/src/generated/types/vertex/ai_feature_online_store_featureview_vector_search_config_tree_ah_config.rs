#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiFeatureOnlineStoreFeatureviewVectorSearchConfigTreeAhConfig {
    /// Number of embeddings on each leaf node. The default value is 1000 if not set.
    #[builder(into)]
    #[serde(rename = "leafNodeEmbeddingCount")]
    pub r#leaf_node_embedding_count: Option<String>,
}
