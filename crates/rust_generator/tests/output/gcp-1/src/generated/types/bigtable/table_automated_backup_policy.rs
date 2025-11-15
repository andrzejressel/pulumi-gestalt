#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableAutomatedBackupPolicy {
    /// How frequently automated backups should occur.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Option<String>,
    /// How long the automated backups should be retained.
    #[builder(into)]
    #[serde(rename = "retentionPeriod")]
    pub r#retention_period: Option<String>,
}
