#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamSourceConfigPostgresqlSourceConfig {
    /// PostgreSQL objects to exclude from the stream.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeObjects")]
    pub r#exclude_objects: Box<Option<super::super::types::datastream::StreamSourceConfigPostgresqlSourceConfigExcludeObjects>>,
    /// PostgreSQL objects to retrieve from the source.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeObjects")]
    pub r#include_objects: Box<Option<super::super::types::datastream::StreamSourceConfigPostgresqlSourceConfigIncludeObjects>>,
    /// Maximum number of concurrent backfill tasks. The number should be non
    /// negative. If not set (or set to 0), the system's default value will be used.
    #[builder(into)]
    #[serde(rename = "maxConcurrentBackfillTasks")]
    pub r#max_concurrent_backfill_tasks: Option<i32>,
    /// The name of the publication that includes the set of all tables
    /// that are defined in the stream's include_objects.
    #[builder(into)]
    #[serde(rename = "publication")]
    pub r#publication: String,
    /// The name of the logical replication slot that's configured with
    /// the pgoutput plugin.
    #[builder(into)]
    #[serde(rename = "replicationSlot")]
    pub r#replication_slot: String,
}
