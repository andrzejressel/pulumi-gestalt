#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEndpointRedshiftSetting {
    #[builder(into)]
    #[serde(rename = "bucketFolder")]
    pub r#bucket_folder: String,
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    #[builder(into)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: String,
    #[builder(into)]
    #[serde(rename = "serverSideEncryptionKmsKeyId")]
    pub r#server_side_encryption_kms_key_id: String,
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: String,
}
