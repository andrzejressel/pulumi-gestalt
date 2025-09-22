#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationBackup {
    /// The retention range in days of the backup policy. Defaults to `5`.
    #[builder(into)]
    #[serde(rename = "instantRpRetentionRangeInDays")]
    pub r#instant_rp_retention_range_in_days: Option<i32>,
    /// The name of the backup policy.
    #[builder(into)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Option<String>,
    /// A `retention_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Option<Box<super::super::types::automanage::ConfigurationBackupRetentionPolicy>>,
    /// A `schedule_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "schedulePolicy")]
    pub r#schedule_policy: Option<Box<super::super::types::automanage::ConfigurationBackupSchedulePolicy>>,
    /// The timezone of the backup policy. Defaults to `UTC`.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}
