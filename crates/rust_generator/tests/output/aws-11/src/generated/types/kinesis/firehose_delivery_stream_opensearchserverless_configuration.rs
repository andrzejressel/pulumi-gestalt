#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamOpensearchserverlessConfiguration {
    /// Buffer incoming data for the specified period of time, in seconds between 0 to 900, before delivering it to the destination.  The default value is 300s.
    #[builder(into)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Option<i32>,
    /// Buffer incoming data to the specified size, in MBs between 1 to 100, before delivering it to the destination.  The default value is 5MB.
    #[builder(into)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Option<i32>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationCloudwatchLoggingOptions>>,
    /// The endpoint to use when communicating with the collection in the Serverless offering for Amazon OpenSearch Service.
    #[builder(into)]
    #[serde(rename = "collectionEndpoint")]
    pub r#collection_endpoint: String,
    /// The Serverless offering for Amazon OpenSearch Service index name.
    #[builder(into)]
    #[serde(rename = "indexName")]
    pub r#index_name: String,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationProcessingConfiguration>>,
    /// After an initial failure to deliver to the Serverless offering for Amazon OpenSearch Service, the total amount of time, in seconds between 0 to 7200, during which Kinesis Data Firehose retries delivery (including the first attempt).  After this time has elapsed, the failed documents are written to Amazon S3.  The default value is 300s.  There will be no retry if the value is 0.
    #[builder(into)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Option<i32>,
    /// The Amazon Resource Name (ARN) of the IAM role to be assumed by Kinesis Data Firehose for calling the Serverless offering for Amazon OpenSearch Service Configuration API and for indexing documents.  The pattern needs to be `arn:.*`.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// Defines how documents should be delivered to Amazon S3.  Valid values are `FailedDocumentsOnly` and `AllDocuments`.  Default value is `FailedDocumentsOnly`.
    #[builder(into)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Option<String>,
    /// The S3 Configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationS3Configuration>,
    /// The VPC configuration for the delivery stream to connect to OpenSearch Serverless associated with the VPC. See `vpc_config` block below for details.
    #[builder(into)]
    #[serde(rename = "vpcConfig")]
    pub r#vpc_config: Box<Option<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationVpcConfig>>,
}
