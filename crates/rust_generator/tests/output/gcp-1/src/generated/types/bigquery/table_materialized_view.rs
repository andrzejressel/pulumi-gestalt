#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableMaterializedView {
    /// Allow non incremental materialized view definition.
    /// The default value is false.
    #[builder(into)]
    #[serde(rename = "allowNonIncrementalDefinition")]
    pub r#allow_non_incremental_definition: Option<bool>,
    /// Specifies whether to use BigQuery's automatic refresh for this materialized view when the base table is updated.
    /// The default value is true.
    #[builder(into)]
    #[serde(rename = "enableRefresh")]
    pub r#enable_refresh: Option<bool>,
    /// A query whose result is persisted.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
    /// The maximum frequency at which this materialized view will be refreshed.
    /// The default value is 1800000
    #[builder(into)]
    #[serde(rename = "refreshIntervalMs")]
    pub r#refresh_interval_ms: Option<i32>,
}
