#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackupPolicyDiskRetentionRuleCriteria {
    /// Possible values are `FirstOfDay` and `FirstOfWeek`. Changing this forces a new Backup Policy Disk to be created.
    #[builder(into)]
    #[serde(rename = "absoluteCriteria")]
    pub r#absolute_criteria: Option<String>,
}
