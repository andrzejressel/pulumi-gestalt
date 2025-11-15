#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamExtendedS3Configuration {
    /// The ARN of the S3 bucket
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: String,
    #[builder(into)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Option<i32>,
    #[builder(into)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationCloudwatchLoggingOptions>>,
    /// The compression format. If no value is specified, the default is `UNCOMPRESSED`. Other supported values are `GZIP`, `ZIP`, `Snappy`, & `HADOOP_SNAPPY`.
    #[builder(into)]
    #[serde(rename = "compressionFormat")]
    pub r#compression_format: Option<String>,
    /// The time zone you prefer. Valid values are `UTC` or a non-3-letter IANA time zones (for example, `America/Los_Angeles`). Default value is `UTC`.
    #[builder(into)]
    #[serde(rename = "customTimeZone")]
    pub r#custom_time_zone: Option<String>,
    /// Nested argument for the serializer, deserializer, and schema for converting data from the JSON format to the Parquet or ORC format before writing it to Amazon S3. See `data_format_conversion_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "dataFormatConversionConfiguration")]
    pub r#data_format_conversion_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfiguration>>,
    /// The configuration for dynamic partitioning. Required when using [dynamic partitioning](https://docs.aws.amazon.com/firehose/latest/dev/dynamic-partitioning.html). See `dynamic_partitioning_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "dynamicPartitioningConfiguration")]
    pub r#dynamic_partitioning_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDynamicPartitioningConfiguration>>,
    /// Prefix added to failed records before writing them to S3. Not currently supported for `redshift` destination. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see [Custom Prefixes for Amazon S3 Objects](https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html).
    #[builder(into)]
    #[serde(rename = "errorOutputPrefix")]
    pub r#error_output_prefix: Option<String>,
    /// The file extension to override the default file extension (for example, `.json`).
    #[builder(into)]
    #[serde(rename = "fileExtension")]
    pub r#file_extension: Option<String>,
    /// Specifies the KMS key ARN the stream will use to encrypt data. If not set, no encryption will
    /// be used.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Option<String>,
    /// The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationProcessingConfiguration>>,
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The configuration for backup in Amazon S3. Required if `s3_backup_mode` is `Enabled`. Supports the same fields as `s3_configuration` object.
    #[builder(into)]
    #[serde(rename = "s3BackupConfiguration")]
    pub r#s_3_backup_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationS3BackupConfiguration>>,
    /// The Amazon S3 backup mode.  Valid values are `Disabled` and `Enabled`.  Default value is `Disabled`.
    #[builder(into)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Option<String>,
}
