#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMasterUserSecret {
    /// ARN for the KMS encryption key. When specifying `kms_key_id`, `storage_encrypted` needs to be set to true.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Amazon Resource Name (ARN) of the secret.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Option<String>,
    /// Status of the secret. Valid Values: `creating` | `active` | `rotating` | `impaired`.
    #[builder(into)]
    #[serde(rename = "secretStatus")]
    pub r#secret_status: Option<String>,
}
