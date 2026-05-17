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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamExtendedS3Configuration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "bucket_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_arn,
                )
                .await,
            );
            map.insert(
                "buffering_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buffering_interval,
                )
                .await,
            );
            map.insert(
                "buffering_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buffering_size,
                )
                .await,
            );
            map.insert(
                "cloudwatch_logging_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloudwatch_logging_options,
                )
                .await,
            );
            map.insert(
                "compression_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compression_format,
                )
                .await,
            );
            map.insert(
                "custom_time_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_time_zone,
                )
                .await,
            );
            map.insert(
                "data_format_conversion_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_format_conversion_configuration,
                )
                .await,
            );
            map.insert(
                "dynamic_partitioning_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dynamic_partitioning_configuration,
                )
                .await,
            );
            map.insert(
                "error_output_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#error_output_prefix,
                )
                .await,
            );
            map.insert(
                "file_extension".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_extension,
                )
                .await,
            );
            map.insert(
                "kms_key_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_key_arn,
                )
                .await,
            );
            map.insert(
                "prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix,
                )
                .await,
            );
            map.insert(
                "processing_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#processing_configuration,
                )
                .await,
            );
            map.insert(
                "role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role_arn,
                )
                .await,
            );
            map.insert(
                "s_3_backup_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_backup_configuration,
                )
                .await,
            );
            map.insert(
                "s_3_backup_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_backup_mode,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamExtendedS3Configuration {
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
                    r#custom_time_zone: {
                        let field_value = match fields_map.get("custom_time_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_time_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_format_conversion_configuration: {
                        let field_value = match fields_map.get("data_format_conversion_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_format_conversion_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_partitioning_configuration: {
                        let field_value = match fields_map.get("dynamic_partitioning_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_partitioning_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#file_extension: {
                        let field_value = match fields_map.get("file_extension") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_extension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#processing_configuration: {
                        let field_value = match fields_map.get("processing_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'processing_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#s_3_backup_configuration: {
                        let field_value = match fields_map.get("s_3_backup_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_backup_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_backup_mode: {
                        let field_value = match fields_map.get("s_3_backup_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_backup_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
