#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDatabaseInstancesInstanceRestoreBackupContext {
    /// The ID of the backup run to restore from.
    #[builder(into)]
    #[serde(rename = "backupRunId")]
    pub r#backup_run_id: i32,
    /// The ID of the instance that the backup was taken from.
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: String,
    /// The ID of the project in which the resources belong. If it is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
}
