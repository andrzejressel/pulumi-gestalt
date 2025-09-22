#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxFunctionAppSlotSiteConfigAppServiceLogs {
    /// The amount of disk space to use for logs. Valid values are between `25` and `100`. Defaults to `35`.
    #[builder(into)]
    #[serde(rename = "diskQuotaMb")]
    pub r#disk_quota_mb: Option<i32>,
    /// The retention period for logs in days. Valid values are between `0` and `99999`.(never delete).
    /// 
    /// > **NOTE:** This block is not supported on Consumption plans.
    #[builder(into)]
    #[serde(rename = "retentionPeriodDays")]
    pub r#retention_period_days: Option<i32>,
}
