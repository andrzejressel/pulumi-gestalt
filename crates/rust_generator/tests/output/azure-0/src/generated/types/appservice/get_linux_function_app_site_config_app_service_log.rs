#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxFunctionAppSiteConfigAppServiceLog {
    /// The amount of disk space used for logs.
    #[builder(into)]
    #[serde(rename = "diskQuotaMb")]
    pub r#disk_quota_mb: i32,
    /// After how many days backups are deleted.
    #[builder(into)]
    #[serde(rename = "retentionPeriodDays")]
    pub r#retention_period_days: i32,
}
