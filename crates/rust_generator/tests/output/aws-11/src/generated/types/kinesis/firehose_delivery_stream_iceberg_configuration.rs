#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamIcebergConfiguration {
    /// Buffer incoming data for the specified period of time, in seconds between 0 and 900, before delivering it to the destination. The default value is 300.
    #[builder(into)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Option<i32>,
    /// Buffer incoming data to the specified size, in MBs between 1 and 128, before delivering it to the destination. The default value is 5.
    #[builder(into)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Option<i32>,
    /// Glue catalog ARN identifier of the destination Apache Iceberg Tables. You must specify the ARN in the format `arn:aws:glue:region:account-id:catalog`
    #[builder(into)]
    #[serde(rename = "catalogArn")]
    pub r#catalog_arn: String,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationCloudwatchLoggingOptions>>,
    /// Destination table configurations which Firehose uses to deliver data to Apache Iceberg Tables. Firehose will write data with insert if table specific configuration is not provided. See `destination_table_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "destinationTableConfigurations")]
    pub r#destination_table_configurations: Option<Vec<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationDestinationTableConfiguration>>,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationProcessingConfiguration>>,
    /// The period of time, in seconds between 0 to 7200, during which Firehose retries to deliver data to the specified destination.
    #[builder(into)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Option<i32>,
    /// The ARN of the IAM role to be assumed by Firehose for calling Apache Iceberg Tables.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    #[builder(into)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Option<String>,
    /// The S3 Configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationS3Configuration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamIcebergConfiguration {
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
                    "bufferingInterval",
                    &self.r#buffering_interval,
                ),
                to_pulumi_object_field(
                    "bufferingSize",
                    &self.r#buffering_size,
                ),
                to_pulumi_object_field(
                    "catalogArn",
                    &self.r#catalog_arn,
                ),
                to_pulumi_object_field(
                    "cloudwatchLoggingOptions",
                    &self.r#cloudwatch_logging_options,
                ),
                to_pulumi_object_field(
                    "destinationTableConfigurations",
                    &self.r#destination_table_configurations,
                ),
                to_pulumi_object_field(
                    "processingConfiguration",
                    &self.r#processing_configuration,
                ),
                to_pulumi_object_field(
                    "retryDuration",
                    &self.r#retry_duration,
                ),
                to_pulumi_object_field(
                    "roleArn",
                    &self.r#role_arn,
                ),
                to_pulumi_object_field(
                    "s3BackupMode",
                    &self.r#s_3_backup_mode,
                ),
                to_pulumi_object_field(
                    "s3Configuration",
                    &self.r#s_3_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamIcebergConfiguration {
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
                    r#buffering_interval: {
                        let field_value = match fields_map.get("bufferingInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'bufferingInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buffering_size: {
                        let field_value = match fields_map.get("bufferingSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'bufferingSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#catalog_arn: {
                        let field_value = match fields_map.get("catalogArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'catalogArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatch_logging_options: {
                        let field_value = match fields_map.get("cloudwatchLoggingOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchLoggingOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_table_configurations: {
                        let field_value = match fields_map.get("destinationTableConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationTableConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#processing_configuration: {
                        let field_value = match fields_map.get("processingConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'processingConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_duration: {
                        let field_value = match fields_map.get("retryDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'retryDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("roleArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_backup_mode: {
                        let field_value = match fields_map.get("s3BackupMode") {
                            Some(value) => value,
                            None => bail!("Missing field 's3BackupMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_configuration: {
                        let field_value = match fields_map.get("s3Configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 's3Configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
