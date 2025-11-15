#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceDynamodbConfigDeltaSyncConfig {
    /// The number of minutes that an Item is stored in the data source.
    #[builder(into)]
    #[serde(rename = "baseTableTtl")]
    pub r#base_table_ttl: Option<i32>,
    /// The table name.
    #[builder(into)]
    #[serde(rename = "deltaSyncTableName")]
    pub r#delta_sync_table_name: String,
    /// The number of minutes that a Delta Sync log entry is stored in the Delta Sync table.
    #[builder(into)]
    #[serde(rename = "deltaSyncTableTtl")]
    pub r#delta_sync_table_ttl: Option<i32>,
}
