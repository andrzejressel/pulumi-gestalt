#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiFeatureOnlineStoreFeatureviewVectorSearchConfig {
    /// Configuration options for using brute force search, which simply implements the standard linear search in the database for each query. It is primarily meant for benchmarking and to generate the ground truth for approximate search.
    #[builder(into)]
    #[serde(rename = "bruteForceConfig")]
    pub r#brute_force_config: Option<Box<super::super::types::vertex::AiFeatureOnlineStoreFeatureviewVectorSearchConfigBruteForceConfig>>,
    /// Column of crowding. This column contains crowding attribute which is a constraint on a neighbor list produced by nearest neighbor search requiring that no more than some value k' of the k neighbors returned have the same value of crowdingAttribute.
    #[builder(into)]
    #[serde(rename = "crowdingColumn")]
    pub r#crowding_column: Option<String>,
    /// The distance measure used in nearest neighbor search.
    /// For details on allowed values, see the [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.featureOnlineStores.featureViews#DistanceMeasureType).
    /// Possible values are: `SQUARED_L2_DISTANCE`, `COSINE_DISTANCE`, `DOT_PRODUCT_DISTANCE`.
    #[builder(into)]
    #[serde(rename = "distanceMeasureType")]
    pub r#distance_measure_type: Option<String>,
    /// Column of embedding. This column contains the source data to create index for vector search.
    #[builder(into)]
    #[serde(rename = "embeddingColumn")]
    pub r#embedding_column: String,
    /// The number of dimensions of the input embedding.
    #[builder(into)]
    #[serde(rename = "embeddingDimension")]
    pub r#embedding_dimension: Option<i32>,
    /// Columns of features that are used to filter vector search results.
    #[builder(into)]
    #[serde(rename = "filterColumns")]
    pub r#filter_columns: Option<Vec<String>>,
    /// Configuration options for the tree-AH algorithm (Shallow tree + Asymmetric Hashing). Please refer to this paper for more details: https://arxiv.org/abs/1908.10396
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "treeAhConfig")]
    pub r#tree_ah_config: Option<Box<super::super::types::vertex::AiFeatureOnlineStoreFeatureviewVectorSearchConfigTreeAhConfig>>,
}
