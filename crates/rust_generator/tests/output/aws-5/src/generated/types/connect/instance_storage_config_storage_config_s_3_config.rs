#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceStorageConfigStorageConfigS3Config {
    /// The S3 bucket name.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    /// The S3 bucket prefix.
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: String,
    /// The encryption configuration. Documented below.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Option<Box<super::super::types::connect::InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig>>,
}
