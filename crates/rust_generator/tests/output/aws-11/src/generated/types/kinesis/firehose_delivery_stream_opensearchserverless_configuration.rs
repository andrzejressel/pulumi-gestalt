#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationCloudwatchLoggingOptions>>,
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
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationProcessingConfiguration>>,
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
    pub r#vpc_config: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationVpcConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamOpensearchserverlessConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
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
                "collection_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#collection_endpoint,
                )
                .await,
            );
            map.insert(
                "index_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#index_name,
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
                "retry_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retry_duration,
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
                "s_3_backup_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_backup_mode,
                )
                .await,
            );
            map.insert(
                "s_3_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_configuration,
                )
                .await,
            );
            map.insert(
                "vpc_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_config,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamOpensearchserverlessConfiguration {
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
                    r#collection_endpoint: {
                        let field_value = match fields_map.get("collection_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'collection_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#index_name: {
                        let field_value = match fields_map.get("index_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'index_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#retry_duration: {
                        let field_value = match fields_map.get("retry_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#s_3_backup_mode: {
                        let field_value = match fields_map.get("s_3_backup_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_backup_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_configuration: {
                        let field_value = match fields_map.get("s_3_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_config: {
                        let field_value = match fields_map.get("vpc_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
