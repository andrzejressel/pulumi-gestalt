#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsFunctionAppBackup {
    /// Is the Backup Job enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// The name of this Windows Function App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A `schedule` block as defined below.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Vec<super::super::types::appservice::GetWindowsFunctionAppBackupSchedule>,
    /// The SAS URL to the container.
    #[builder(into)]
    #[serde(rename = "storageAccountUrl")]
    pub r#storage_account_url: String,
}
