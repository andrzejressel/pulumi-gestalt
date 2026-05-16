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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("buffering_interval".to_string(), self.r#buffering_interval.to_pulumi_value().await);
            map.insert("buffering_size".to_string(), self.r#buffering_size.to_pulumi_value().await);
            map.insert("catalog_arn".to_string(), self.r#catalog_arn.to_pulumi_value().await);
            map.insert("cloudwatch_logging_options".to_string(), self.r#cloudwatch_logging_options.to_pulumi_value().await);
            map.insert("destination_table_configurations".to_string(), self.r#destination_table_configurations.to_pulumi_value().await);
            map.insert("processing_configuration".to_string(), self.r#processing_configuration.to_pulumi_value().await);
            map.insert("retry_duration".to_string(), self.r#retry_duration.to_pulumi_value().await);
            map.insert("role_arn".to_string(), self.r#role_arn.to_pulumi_value().await);
            map.insert("s_3_backup_mode".to_string(), self.r#s_3_backup_mode.to_pulumi_value().await);
            map.insert("s_3_configuration".to_string(), self.r#s_3_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamIcebergConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#buffering_interval: {
                        let field_value = match fields_map.get("buffering_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffering_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#buffering_size: {
                        let field_value = match fields_map.get("buffering_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffering_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#catalog_arn: {
                        let field_value = match fields_map.get("catalog_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'catalog_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cloudwatch_logging_options: {
                        let field_value = match fields_map.get("cloudwatch_logging_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logging_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationCloudwatchLoggingOptions>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#destination_table_configurations: {
                        let field_value = match fields_map.get("destination_table_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_table_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationDestinationTableConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#processing_configuration: {
                        let field_value = match fields_map.get("processing_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'processing_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationProcessingConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#retry_duration: {
                        let field_value = match fields_map.get("retry_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_backup_mode: {
                        let field_value = match fields_map.get("s_3_backup_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_backup_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_configuration: {
                        let field_value = match fields_map.get("s_3_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfigurationS3Configuration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
