#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigOracleSourceConfig {
    /// Configuration to drop large object values.
    #[builder(into)]
    #[serde(rename = "dropLargeObjects")]
    pub r#drop_large_objects: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfigDropLargeObjects>>,
    /// Oracle objects to exclude from the stream.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeObjects")]
    pub r#exclude_objects: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfigExcludeObjects>>,
    /// Oracle objects to retrieve from the source.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeObjects")]
    pub r#include_objects: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfigIncludeObjects>>,
    /// Maximum number of concurrent backfill tasks. The number should be non negative.
    /// If not set (or set to 0), the system's default value will be used.
    #[builder(into)]
    #[serde(rename = "maxConcurrentBackfillTasks")]
    pub r#max_concurrent_backfill_tasks: Option<i32>,
    /// Maximum number of concurrent CDC tasks. The number should be non negative.
    /// If not set (or set to 0), the system's default value will be used.
    #[builder(into)]
    #[serde(rename = "maxConcurrentCdcTasks")]
    pub r#max_concurrent_cdc_tasks: Option<i32>,
    /// Configuration to drop large object values.
    #[builder(into)]
    #[serde(rename = "streamLargeObjects")]
    pub r#stream_large_objects: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfigStreamLargeObjects>>,
}
