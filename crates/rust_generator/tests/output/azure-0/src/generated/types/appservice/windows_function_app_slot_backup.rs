#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppSlotBackup {
    /// Should this backup job be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The name which should be used for this Backup.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// a `schedule` block as detailed below.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<super::super::types::appservice::WindowsFunctionAppSlotBackupSchedule>,
    /// The SAS URL to the container.
    #[builder(into)]
    #[serde(rename = "storageAccountUrl")]
    pub r#storage_account_url: String,
}
