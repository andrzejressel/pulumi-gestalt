#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamOpensearchserverlessConfiguration {
    /// Buffer incoming data for the specified period of time, in seconds between 0 to 900, before delivering it to the destination.  The default value is 300s.
    #[builder(into)]
    pub r#buffering_interval: Option<i32>,
    /// Buffer incoming data to the specified size, in MBs between 1 to 100, before delivering it to the destination.  The default value is 5MB.
    #[builder(into)]
    pub r#buffering_size: Option<i32>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into)]
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationCloudwatchLoggingOptions>>,
    /// The endpoint to use when communicating with the collection in the Serverless offering for Amazon OpenSearch Service.
    #[builder(into)]
    pub r#collection_endpoint: String,
    /// The Serverless offering for Amazon OpenSearch Service index name.
    #[builder(into)]
    pub r#index_name: String,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into)]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationProcessingConfiguration>>,
    /// After an initial failure to deliver to the Serverless offering for Amazon OpenSearch Service, the total amount of time, in seconds between 0 to 7200, during which Kinesis Data Firehose retries delivery (including the first attempt).  After this time has elapsed, the failed documents are written to Amazon S3.  The default value is 300s.  There will be no retry if the value is 0.
    #[builder(into)]
    pub r#retry_duration: Option<i32>,
    /// The Amazon Resource Name (ARN) of the IAM role to be assumed by Kinesis Data Firehose for calling the Serverless offering for Amazon OpenSearch Service Configuration API and for indexing documents.  The pattern needs to be `arn:.*`.
    #[builder(into)]
    pub r#role_arn: String,
    /// Defines how documents should be delivered to Amazon S3.  Valid values are `FailedDocumentsOnly` and `AllDocuments`.  Default value is `FailedDocumentsOnly`.
    #[builder(into)]
    pub r#s_3_backup_mode: Option<String>,
    /// The S3 Configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationS3Configuration>,
    /// The VPC configuration for the delivery stream to connect to OpenSearch Serverless associated with the VPC. See `vpc_config` block below for details.
    #[builder(into)]
    pub r#vpc_config: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfigurationVpcConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamOpensearchserverlessConfiguration {
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
                    "cloudwatchLoggingOptions",
                    &self.r#cloudwatch_logging_options,
                ),
                to_pulumi_object_field(
                    "collectionEndpoint",
                    &self.r#collection_endpoint,
                ),
                to_pulumi_object_field(
                    "indexName",
                    &self.r#index_name,
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
                to_pulumi_object_field(
                    "vpcConfig",
                    &self.r#vpc_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                    r#cloudwatch_logging_options: {
                        let field_value = match fields_map.get("cloudwatchLoggingOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchLoggingOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#collection_endpoint: {
                        let field_value = match fields_map.get("collectionEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'collectionEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#index_name: {
                        let field_value = match fields_map.get("indexName") {
                            Some(value) => value,
                            None => bail!("Missing field 'indexName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#vpc_config: {
                        let field_value = match fields_map.get("vpcConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
