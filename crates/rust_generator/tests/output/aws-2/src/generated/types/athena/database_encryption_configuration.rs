#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseEncryptionConfiguration {
    /// Type of key; one of `SSE_S3`, `SSE_KMS`, `CSE_KMS`
    #[builder(into)]
    #[serde(rename = "encryptionOption")]
    pub r#encryption_option: String,
    /// KMS key ARN or ID; required for key types `SSE_KMS` and `CSE_KMS`.
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Option<String>,
}
