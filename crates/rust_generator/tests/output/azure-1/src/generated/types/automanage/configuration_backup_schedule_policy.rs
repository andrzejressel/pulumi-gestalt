#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationBackupSchedulePolicy {
    /// The schedule policy type of the backup policy. Possible value is `SimpleSchedulePolicy`. Defaults to `SimpleSchedulePolicy`.
    #[builder(into)]
    #[serde(rename = "schedulePolicyType")]
    pub r#schedule_policy_type: Option<String>,
    /// The schedule run days of the backup policy. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` and `Saturday`.
    #[builder(into)]
    #[serde(rename = "scheduleRunDays")]
    pub r#schedule_run_days: Option<Vec<String>>,
    /// The schedule run frequency of the backup policy. Possible values are `Daily` and `Weekly`. Defaults to `Daily`.
    #[builder(into)]
    #[serde(rename = "scheduleRunFrequency")]
    pub r#schedule_run_frequency: Option<String>,
    /// The schedule run times of the backup policy.
    #[builder(into)]
    #[serde(rename = "scheduleRunTimes")]
    pub r#schedule_run_times: Option<Vec<String>>,
}
