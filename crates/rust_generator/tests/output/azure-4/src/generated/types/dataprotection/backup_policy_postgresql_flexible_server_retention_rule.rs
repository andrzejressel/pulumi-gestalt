#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackupPolicyPostgresqlFlexibleServerRetentionRule {
    /// A `criteria` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "criteria")]
    pub r#criteria: Box<super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerRetentionRuleCriteria>,
    /// A `life_cycle` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "lifeCycles")]
    pub r#life_cycles: Vec<super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerRetentionRuleLifeCycle>,
    /// Specifies the name of the retention rule. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the priority of the rule. The priority number must be unique for each rule. The lower the priority number, the higher the priority of the rule. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
}
