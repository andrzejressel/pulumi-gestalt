#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSourceBackupConfigInfoGcpBackupConfig {
    /// The name of the backup plan.
    #[builder(into)]
    #[serde(rename = "backupPlan")]
    pub r#backup_plan: String,
    /// The name of the backup plan association.
    #[builder(into)]
    #[serde(rename = "backupPlanAssociation")]
    pub r#backup_plan_association: String,
    /// The description of the backup plan.
    #[builder(into)]
    #[serde(rename = "backupPlanDescription")]
    pub r#backup_plan_description: String,
    /// The names of the backup plan rules which point to this backupvault
    #[builder(into)]
    #[serde(rename = "backupPlanRules")]
    pub r#backup_plan_rules: Vec<String>,
}
