#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelPrefixLevelStorageMetricsSelectionCriteria {
    /// The delimiter of the selection criteria being used.
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Option<String>,
    /// The max depth of the selection criteria.
    #[builder(into)]
    #[serde(rename = "maxDepth")]
    pub r#max_depth: Option<i32>,
    /// The minimum number of storage bytes percentage whose metrics will be selected.
    #[builder(into)]
    #[serde(rename = "minStorageBytesPercentage")]
    pub r#min_storage_bytes_percentage: Option<f64>,
}
