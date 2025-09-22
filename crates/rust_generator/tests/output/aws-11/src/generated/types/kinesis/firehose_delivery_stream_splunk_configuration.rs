#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamSplunkConfiguration {
    /// Buffer incoming data for the specified period of time, in seconds between 0 to 60, before delivering it to the destination.  The default value is 60s.
    #[builder(into)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Option<i32>,
    /// Buffer incoming data to the specified size, in MBs between 1 to 5, before delivering it to the destination.  The default value is 5MB.
    #[builder(into)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Option<i32>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamSplunkConfigurationCloudwatchLoggingOptions>>,
    /// The amount of time, in seconds between 180 and 600, that Kinesis Firehose waits to receive an acknowledgment from Splunk after it sends it data.
    #[builder(into)]
    #[serde(rename = "hecAcknowledgmentTimeout")]
    pub r#hec_acknowledgment_timeout: Option<i32>,
    /// The HTTP Event Collector (HEC) endpoint to which Kinesis Firehose sends your data.
    #[builder(into)]
    #[serde(rename = "hecEndpoint")]
    pub r#hec_endpoint: String,
    /// The HEC endpoint type. Valid values are `Raw` or `Event`. The default value is `Raw`.
    #[builder(into)]
    #[serde(rename = "hecEndpointType")]
    pub r#hec_endpoint_type: Option<String>,
    /// The GUID that you obtain from your Splunk cluster when you create a new HEC endpoint. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into)]
    #[serde(rename = "hecToken")]
    pub r#hec_token: Option<String>,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamSplunkConfigurationProcessingConfiguration>>,
    /// After an initial failure to deliver to Splunk, the total amount of time, in seconds between 0 to 7200, during which Firehose re-attempts delivery (including the first attempt).  After this time has elapsed, the failed documents are written to Amazon S3.  The default value is 300s.  There will be no retry if the value is 0.
    #[builder(into)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Option<i32>,
    /// Defines how documents should be delivered to Amazon S3.  Valid values are `FailedEventsOnly` and `AllEvents`.  Default value is `FailedEventsOnly`.
    /// `secrets_manager_configuration` - (Optional) The Secrets Manager configuration. See `secrets_manager_configuration` block below for details. This value is required if `hec_token` is not provided.
    #[builder(into)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Option<String>,
    /// The S3 Configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamSplunkConfigurationS3Configuration>,
    #[builder(into)]
    #[serde(rename = "secretsManagerConfiguration")]
    pub r#secrets_manager_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamSplunkConfigurationSecretsManagerConfiguration>>,
}
