#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTableRetentionProperty {
    /// Duration in days in which the data must be stored in magnetic store.
    #[builder(into)]
    #[serde(rename = "magneticStoreRetentionPeriodInDays")]
    pub r#magnetic_store_retention_period_in_days: i32,
    /// Duration in hours in which the data must be stored in memory store.
    #[builder(into)]
    #[serde(rename = "memoryStoreRetentionPeriodInHours")]
    pub r#memory_store_retention_period_in_hours: i32,
}
