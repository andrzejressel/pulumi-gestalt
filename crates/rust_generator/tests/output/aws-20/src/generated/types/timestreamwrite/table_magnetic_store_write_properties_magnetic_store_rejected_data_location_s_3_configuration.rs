#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration {
    /// Bucket name of the customer S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Option<String>,
    /// Encryption option for the customer s3 location. Options are S3 server side encryption with an S3-managed key or KMS managed key. Valid values are `SSE_KMS` and `SSE_S3`.
    #[builder(into)]
    #[serde(rename = "encryptionOption")]
    pub r#encryption_option: Option<String>,
    /// KMS key arn for the customer s3 location when encrypting with a KMS managed key.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Object key prefix for the customer S3 location.
    #[builder(into)]
    #[serde(rename = "objectKeyPrefix")]
    pub r#object_key_prefix: Option<String>,
}
