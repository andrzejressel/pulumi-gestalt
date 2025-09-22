#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SnapshotEncryptionSettings {
    /// A `disk_encryption_key` block as defined below.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKey")]
    pub r#disk_encryption_key: Box<super::super::types::compute::SnapshotEncryptionSettingsDiskEncryptionKey>,
    /// A `key_encryption_key` block as defined below.
    #[builder(into)]
    #[serde(rename = "keyEncryptionKey")]
    pub r#key_encryption_key: Box<Option<super::super::types::compute::SnapshotEncryptionSettingsKeyEncryptionKey>>,
}
