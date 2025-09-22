#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PlanRule {
    /// The amount of time in minutes AWS Backup attempts a backup before canceling the job and returning an error.
    #[builder(into)]
    #[serde(rename = "completionWindow")]
    pub r#completion_window: Option<i32>,
    /// Configuration block(s) with copy operation settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "copyActions")]
    pub r#copy_actions: Option<Vec<super::super::types::backup::PlanRuleCopyAction>>,
    /// Enable continuous backups for supported resources.
    #[builder(into)]
    #[serde(rename = "enableContinuousBackup")]
    pub r#enable_continuous_backup: Option<bool>,
    /// The lifecycle defines when a protected resource is transitioned to cold storage and when it expires.  Fields documented below.
    #[builder(into)]
    #[serde(rename = "lifecycle")]
    pub r#lifecycle: Option<Box<super::super::types::backup::PlanRuleLifecycle>>,
    /// Metadata that you can assign to help organize the resources that you create.
    #[builder(into)]
    #[serde(rename = "recoveryPointTags")]
    pub r#recovery_point_tags: Option<std::collections::HashMap<String, String>>,
    /// An display name for a backup rule.
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: String,
    /// A CRON expression specifying when AWS Backup initiates a backup job.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
    /// The timezone in which the schedule expression is set. Default value: `"Etc/UTC"`.
    #[builder(into)]
    #[serde(rename = "scheduleExpressionTimezone")]
    pub r#schedule_expression_timezone: Option<String>,
    /// The amount of time in minutes before beginning a backup.
    #[builder(into)]
    #[serde(rename = "startWindow")]
    pub r#start_window: Option<i32>,
    /// The name of a logical container where backups are stored.
    #[builder(into)]
    #[serde(rename = "targetVaultName")]
    pub r#target_vault_name: String,
}
