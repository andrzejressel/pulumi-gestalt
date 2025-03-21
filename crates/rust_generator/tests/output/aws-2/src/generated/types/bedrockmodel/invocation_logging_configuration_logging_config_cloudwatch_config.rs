#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InvocationLoggingConfigurationLoggingConfigCloudwatchConfig {
    /// S3 configuration for delivering a large amount of data.
    #[builder(into, default)]
    #[serde(rename = "largeDataDeliveryS3Config")]
    pub r#large_data_delivery_s_3_config: Box<Option<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigCloudwatchConfigLargeDataDeliveryS3Config>>,
    /// Log group name.
    #[builder(into, default)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Box<Option<String>>,
    /// The role ARN.
    #[builder(into, default)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<Option<String>>,
}
