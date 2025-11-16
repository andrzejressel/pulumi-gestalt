#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InvocationLoggingConfigurationLoggingConfig {
    /// CloudWatch logging configuration.
    #[builder(into)]
    #[serde(rename = "cloudwatchConfig")]
    pub r#cloudwatch_config: Option<Box<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigCloudwatchConfig>>,
    /// Set to include embeddings data in the log delivery.
    #[builder(into)]
    #[serde(rename = "embeddingDataDeliveryEnabled")]
    pub r#embedding_data_delivery_enabled: bool,
    /// Set to include image data in the log delivery.
    #[builder(into)]
    #[serde(rename = "imageDataDeliveryEnabled")]
    pub r#image_data_delivery_enabled: bool,
    /// S3 configuration for storing log data.
    #[builder(into)]
    #[serde(rename = "s3Config")]
    pub r#s_3_config: Option<Box<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigS3Config>>,
    /// Set to include text data in the log delivery.
    #[builder(into)]
    #[serde(rename = "textDataDeliveryEnabled")]
    pub r#text_data_delivery_enabled: bool,
}
