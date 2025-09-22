#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointConfigurationDataCaptureConfig {
    /// The content type headers to capture. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "captureContentTypeHeader")]
    pub r#capture_content_type_header: Box<Option<super::super::types::sagemaker::EndpointConfigurationDataCaptureConfigCaptureContentTypeHeader>>,
    /// Specifies what data to capture. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "captureOptions")]
    pub r#capture_options: Vec<super::super::types::sagemaker::EndpointConfigurationDataCaptureConfigCaptureOption>,
    /// The URL for S3 location where the captured data is stored.
    #[builder(into)]
    #[serde(rename = "destinationS3Uri")]
    pub r#destination_s_3_uri: String,
    /// Flag to enable data capture. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableCapture")]
    pub r#enable_capture: Option<bool>,
    /// Portion of data to capture. Should be between 0 and 100.
    #[builder(into)]
    #[serde(rename = "initialSamplingPercentage")]
    pub r#initial_sampling_percentage: i32,
    /// Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt the captured data on Amazon S3.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
}
