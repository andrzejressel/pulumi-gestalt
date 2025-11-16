#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterConfigurationManagedStorageConfiguration {
    /// AWS Key Management Service key ID for the Fargate ephemeral storage.
    #[builder(into)]
    #[serde(rename = "fargateEphemeralStorageKmsKeyId")]
    pub r#fargate_ephemeral_storage_kms_key_id: Option<String>,
    /// AWS Key Management Service key ID to encrypt the managed storage.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
}
