#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseInstanceSettingsBackupConfigurationBackupRetentionSettings {
    /// Depending on the value of retention_unit, this is used to determine if a backup needs to be deleted. If retention_unit
    /// is 'COUNT', we will retain this many backups.
    #[builder(into)]
    #[serde(rename = "retainedBackups")]
    pub r#retained_backups: i32,
    /// The unit that 'retained_backups' represents. Defaults to `COUNT`.
    #[builder(into)]
    #[serde(rename = "retentionUnit")]
    pub r#retention_unit: Option<String>,
}
