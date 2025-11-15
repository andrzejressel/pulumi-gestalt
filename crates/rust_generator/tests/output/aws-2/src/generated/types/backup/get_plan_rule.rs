#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPlanRule {
    #[builder(into)]
    #[serde(rename = "completionWindow")]
    pub r#completion_window: i32,
    #[builder(into)]
    #[serde(rename = "copyActions")]
    pub r#copy_actions: Vec<super::super::types::backup::GetPlanRuleCopyAction>,
    #[builder(into)]
    #[serde(rename = "enableContinuousBackup")]
    pub r#enable_continuous_backup: bool,
    #[builder(into)]
    #[serde(rename = "lifecycles")]
    pub r#lifecycles: Vec<super::super::types::backup::GetPlanRuleLifecycle>,
    #[builder(into)]
    #[serde(rename = "recoveryPointTags")]
    pub r#recovery_point_tags: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: String,
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: String,
    #[builder(into)]
    #[serde(rename = "scheduleExpressionTimezone")]
    pub r#schedule_expression_timezone: String,
    #[builder(into)]
    #[serde(rename = "startWindow")]
    pub r#start_window: i32,
    #[builder(into)]
    #[serde(rename = "targetVaultName")]
    pub r#target_vault_name: String,
}
