#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryCreationTemplateEncryptionConfiguration {
    /// The encryption type to use for any created repositories. Valid values are `AES256` or `KMS`. Defaults to `AES256`.
    #[builder(into, default)]
    #[serde(rename = "encryptionType")]
    pub r#encryption_type: Box<Option<String>>,
    /// The ARN of the KMS key to use when `encryption_type` is `KMS`. If not specified, uses the default AWS managed key for ECR.
    #[builder(into, default)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Box<Option<String>>,
}
