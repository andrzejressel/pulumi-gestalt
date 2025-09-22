#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamSourceConfigMysqlSourceConfig {
    /// MySQL objects to exclude from the stream.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeObjects")]
    pub r#exclude_objects: Option<Box<super::super::types::datastream::StreamSourceConfigMysqlSourceConfigExcludeObjects>>,
    /// MySQL objects to retrieve from the source.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeObjects")]
    pub r#include_objects: Option<Box<super::super::types::datastream::StreamSourceConfigMysqlSourceConfigIncludeObjects>>,
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
}
