#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterContinuousBackupInfoEncryptionInfo {
    /// (Output)
    /// Output only. Type of encryption.
    #[builder(into, default)]
    #[serde(rename = "encryptionType")]
    pub r#encryption_type: Box<Option<String>>,
    /// (Output)
    /// Output only. Cloud KMS key versions that are being used to protect the database or the backup.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyVersions")]
    pub r#kms_key_versions: Box<Option<Vec<String>>>,
}
