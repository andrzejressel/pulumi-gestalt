#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamIcebergConfigurationS3Configuration {
    /// The ARN of the S3 bucket
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: String,
    /// Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 300.
    #[builder(into)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Option<i32>,
    /// Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 5.
    /// We recommend setting SizeInMBs to a value greater than the amount of data you typically ingest into the delivery stream in 10 seconds. For example, if you typically ingest data at 1 MB/sec set SizeInMBs to be 10 MB or higher.
    #[builder(into)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Option<i32>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationS3ConfigurationCloudwatchLoggingOptions>>,
    /// The compression format. If no value is specified, the default is `UNCOMPRESSED`. Other supported values are `GZIP`, `ZIP`, `Snappy`, & `HADOOP_SNAPPY`.
    #[builder(into)]
    #[serde(rename = "compressionFormat")]
    pub r#compression_format: Option<String>,
    /// Prefix added to failed records before writing them to S3. Not currently supported for `redshift` destination. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see [Custom Prefixes for Amazon S3 Objects](https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html).
    #[builder(into)]
    #[serde(rename = "errorOutputPrefix")]
    pub r#error_output_prefix: Option<String>,
    /// Specifies the KMS key ARN the stream will use to encrypt data. If not set, no encryption will
    /// be used.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Option<String>,
    /// The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// The ARN of the AWS credentials.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamIcebergConfigurationS3Configuration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "bucket_arn",
                    &self.r#bucket_arn,
                ),
                to_pulumi_object_field(
                    "buffering_interval",
                    &self.r#buffering_interval,
                ),
                to_pulumi_object_field(
                    "buffering_size",
                    &self.r#buffering_size,
                ),
                to_pulumi_object_field(
                    "cloudwatch_logging_options",
                    &self.r#cloudwatch_logging_options,
                ),
                to_pulumi_object_field(
                    "compression_format",
                    &self.r#compression_format,
                ),
                to_pulumi_object_field(
                    "error_output_prefix",
                    &self.r#error_output_prefix,
                ),
                to_pulumi_object_field(
                    "kms_key_arn",
                    &self.r#kms_key_arn,
                ),
                to_pulumi_object_field(
                    "prefix",
                    &self.r#prefix,
                ),
                to_pulumi_object_field(
                    "role_arn",
                    &self.r#role_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamIcebergConfigurationS3Configuration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#bucket_arn: {
                        let field_value = match fields_map.get("bucket_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buffering_interval: {
                        let field_value = match fields_map.get("buffering_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffering_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buffering_size: {
                        let field_value = match fields_map.get("buffering_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffering_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatch_logging_options: {
                        let field_value = match fields_map.get("cloudwatch_logging_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logging_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compression_format: {
                        let field_value = match fields_map.get("compression_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error_output_prefix: {
                        let field_value = match fields_map.get("error_output_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_output_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_arn: {
                        let field_value = match fields_map.get("kms_key_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
