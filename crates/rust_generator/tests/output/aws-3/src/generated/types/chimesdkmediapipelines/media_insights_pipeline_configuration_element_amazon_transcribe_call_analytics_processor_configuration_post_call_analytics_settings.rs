#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings {
    /// Should output be redacted.
    #[builder(into)]
    #[serde(rename = "contentRedactionOutput")]
    pub r#content_redaction_output: Option<String>,
    /// ARN of the role used by AWS Transcribe to upload your post call analysis.
    #[builder(into)]
    #[serde(rename = "dataAccessRoleArn")]
    pub r#data_access_role_arn: String,
    /// ID of the KMS key used to encrypt the output.
    #[builder(into)]
    #[serde(rename = "outputEncryptionKmsKeyId")]
    pub r#output_encryption_kms_key_id: Option<String>,
    /// The Amazon S3 location where you want your Call Analytics post-call transcription output stored.
    #[builder(into)]
    #[serde(rename = "outputLocation")]
    pub r#output_location: String,
}
