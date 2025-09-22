#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointRedshiftSettings {
    /// Custom S3 Bucket Object prefix for intermediate storage.
    #[builder(into)]
    #[serde(rename = "bucketFolder")]
    pub r#bucket_folder: Option<String>,
    /// Custom S3 Bucket name for intermediate storage.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Option<String>,
    /// The server-side encryption mode that you want to encrypt your intermediate .csv object files copied to S3. Defaults to `SSE_S3`. Valid values are `SSE_S3` and `SSE_KMS`.
    #[builder(into)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: Option<String>,
    /// ARN or Id of KMS Key to use when `encryption_mode` is `SSE_KMS`.
    #[builder(into)]
    #[serde(rename = "serverSideEncryptionKmsKeyId")]
    pub r#server_side_encryption_kms_key_id: Option<String>,
    /// Amazon Resource Name (ARN) of the IAM Role with permissions to read from or write to the S3 Bucket for intermediate storage.
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Option<String>,
}
