#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketServerSideEncryptionConfigurationRuleApplyServerSideEncryptionByDefault {
    /// The AWS KMS master key ID used for the SSE-KMS encryption. This can only be used when you set the value of `sse_algorithm` as `aws:kms`. The default `aws/s3` AWS KMS master key is used if this element is absent while the `sse_algorithm` is `aws:kms`.
    #[builder(into, default)]
    #[serde(rename = "kmsMasterKeyId")]
    pub r#kms_master_key_id: Box<Option<String>>,
    /// The server-side encryption algorithm to use. Valid values are `AES256` and `aws:kms`
    #[builder(into)]
    #[serde(rename = "sseAlgorithm")]
    pub r#sse_algorithm: Box<String>,
}
