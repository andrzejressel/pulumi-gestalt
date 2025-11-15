#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamHttpEndpointConfiguration {
    /// The access key required for Kinesis Firehose to authenticate with the HTTP endpoint selected as the destination.
    #[builder(into)]
    #[serde(rename = "accessKey")]
    pub r#access_key: Option<String>,
    /// Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 300 (5 minutes).
    #[builder(into)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Option<i32>,
    /// Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 5.
    #[builder(into)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Option<i32>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfigurationCloudwatchLoggingOptions>>,
    /// The HTTP endpoint name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfigurationProcessingConfiguration>>,
    /// The request configuration.  See `request_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "requestConfiguration")]
    pub r#request_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfiguration>>,
    /// Total amount of seconds Firehose spends on retries. This duration starts after the initial attempt fails, It does not include the time periods during which Firehose waits for acknowledgment from the specified destination after each attempt. Valid values between `0` and `7200`. Default is `300`.
    #[builder(into)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Option<i32>,
    /// Kinesis Data Firehose uses this IAM role for all the permissions that the delivery stream needs. The pattern needs to be `arn:.*`.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
    /// Defines how documents should be delivered to Amazon S3.  Valid values are `FailedDataOnly` and `AllData`.  Default value is `FailedDataOnly`.
    #[builder(into)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Option<String>,
    /// The S3 Configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfigurationS3Configuration>,
    /// The Secret Manager Configuration. See `secrets_manager_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "secretsManagerConfiguration")]
    pub r#secrets_manager_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfigurationSecretsManagerConfiguration>>,
    /// The HTTP endpoint URL to which Kinesis Firehose sends your data.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
