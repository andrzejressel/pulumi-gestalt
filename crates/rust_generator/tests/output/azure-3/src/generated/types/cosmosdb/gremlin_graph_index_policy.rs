#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GremlinGraphIndexPolicy {
    /// Indicates if the indexing policy is automatic. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "automatic")]
    pub r#automatic: Option<bool>,
    /// One or more `composite_index` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "compositeIndices")]
    pub r#composite_indices: Option<Vec<super::super::types::cosmosdb::GremlinGraphIndexPolicyCompositeIndex>>,
    /// List of paths to exclude from indexing. Required if `indexing_mode` is `Consistent` or `Lazy`.
    #[builder(into)]
    #[serde(rename = "excludedPaths")]
    pub r#excluded_paths: Option<Vec<String>>,
    /// List of paths to include in the indexing. Required if `indexing_mode` is `Consistent` or `Lazy`.
    #[builder(into)]
    #[serde(rename = "includedPaths")]
    pub r#included_paths: Option<Vec<String>>,
    /// Indicates the indexing mode. Possible values include: `Consistent`, `Lazy`, `None`.
    #[builder(into)]
    #[serde(rename = "indexingMode")]
    pub r#indexing_mode: String,
    /// One or more `spatial_index` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "spatialIndices")]
    pub r#spatial_indices: Option<Vec<super::super::types::cosmosdb::GremlinGraphIndexPolicySpatialIndex>>,
}
