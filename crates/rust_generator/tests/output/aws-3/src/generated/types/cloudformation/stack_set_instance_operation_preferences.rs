#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StackSetInstanceOperationPreferences {
    /// Specifies how the concurrency level behaves during the operation execution. Valid values are `STRICT_FAILURE_TOLERANCE` and `SOFT_FAILURE_TOLERANCE`.
    #[builder(into)]
    #[serde(rename = "concurrencyMode")]
    pub r#concurrency_mode: Option<String>,
    /// Number of accounts, per Region, for which this operation can fail before AWS CloudFormation stops the operation in that Region.
    #[builder(into)]
    #[serde(rename = "failureToleranceCount")]
    pub r#failure_tolerance_count: Option<i32>,
    /// Percentage of accounts, per Region, for which this stack operation can fail before AWS CloudFormation stops the operation in that Region.
    #[builder(into)]
    #[serde(rename = "failureTolerancePercentage")]
    pub r#failure_tolerance_percentage: Option<i32>,
    /// Maximum number of accounts in which to perform this operation at one time.
    #[builder(into)]
    #[serde(rename = "maxConcurrentCount")]
    pub r#max_concurrent_count: Option<i32>,
    /// Maximum percentage of accounts in which to perform this operation at one time.
    #[builder(into)]
    #[serde(rename = "maxConcurrentPercentage")]
    pub r#max_concurrent_percentage: Option<i32>,
    /// Concurrency type of deploying StackSets operations in Regions, could be in parallel or one Region at a time. Valid values are `SEQUENTIAL` and `PARALLEL`.
    #[builder(into)]
    #[serde(rename = "regionConcurrencyType")]
    pub r#region_concurrency_type: Option<String>,
    /// Order of the Regions in where you want to perform the stack operation.
    #[builder(into)]
    #[serde(rename = "regionOrders")]
    pub r#region_orders: Option<Vec<String>>,
}
