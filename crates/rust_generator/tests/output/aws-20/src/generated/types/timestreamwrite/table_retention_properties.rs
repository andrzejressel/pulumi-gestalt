#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableRetentionProperties {
    /// The duration for which data must be stored in the magnetic store. Minimum value of 1. Maximum value of 73000.
    #[builder(into)]
    #[serde(rename = "magneticStoreRetentionPeriodInDays")]
    pub r#magnetic_store_retention_period_in_days: Box<i32>,
    /// The duration for which data must be stored in the memory store. Minimum value of 1. Maximum value of 8766.
    #[builder(into)]
    #[serde(rename = "memoryStoreRetentionPeriodInHours")]
    pub r#memory_store_retention_period_in_hours: Box<i32>,
}
