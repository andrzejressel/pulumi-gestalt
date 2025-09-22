#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityConfigurationEncryptionConfigurationCloudwatchEncryption {
    /// Encryption mode to use for CloudWatch data. Valid values: `DISABLED`, `SSE-KMS`. Default value: `DISABLED`.
    #[builder(into)]
    #[serde(rename = "cloudwatchEncryptionMode")]
    pub r#cloudwatch_encryption_mode: Option<String>,
    /// Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Option<String>,
}
