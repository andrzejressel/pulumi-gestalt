#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SqlContainerIndexingPolicy {
    /// One or more `composite_index` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "compositeIndices")]
    pub r#composite_indices: Option<Vec<super::super::types::cosmosdb::SqlContainerIndexingPolicyCompositeIndex>>,
    /// One or more `excluded_path` blocks as defined below. Either `included_path` or `excluded_path` must contain the `path` `/*`
    #[builder(into)]
    #[serde(rename = "excludedPaths")]
    pub r#excluded_paths: Option<Vec<super::super::types::cosmosdb::SqlContainerIndexingPolicyExcludedPath>>,
    /// One or more `included_path` blocks as defined below. Either `included_path` or `excluded_path` must contain the `path` `/*`
    #[builder(into)]
    #[serde(rename = "includedPaths")]
    pub r#included_paths: Option<Vec<super::super::types::cosmosdb::SqlContainerIndexingPolicyIncludedPath>>,
    /// Indicates the indexing mode. Possible values include: `consistent` and `none`. Defaults to `consistent`.
    #[builder(into)]
    #[serde(rename = "indexingMode")]
    pub r#indexing_mode: Option<String>,
    /// One or more `spatial_index` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "spatialIndices")]
    pub r#spatial_indices: Option<Vec<super::super::types::cosmosdb::SqlContainerIndexingPolicySpatialIndex>>,
}
