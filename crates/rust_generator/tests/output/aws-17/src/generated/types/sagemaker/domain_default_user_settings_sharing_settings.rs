#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultUserSettingsSharingSettings {
    /// Whether to include the notebook cell output when sharing the notebook. The default is `Disabled`. Valid values are `Allowed` and `Disabled`.
    #[builder(into)]
    #[serde(rename = "notebookOutputOption")]
    pub r#notebook_output_option: Option<String>,
    /// When `notebook_output_option` is Allowed, the AWS Key Management Service (KMS) encryption key ID used to encrypt the notebook cell output in the Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3KmsKeyId")]
    pub r#s_3_kms_key_id: Option<String>,
    /// When `notebook_output_option` is Allowed, the Amazon S3 bucket used to save the notebook cell output.
    #[builder(into)]
    #[serde(rename = "s3OutputPath")]
    pub r#s_3_output_path: Option<String>,
}
