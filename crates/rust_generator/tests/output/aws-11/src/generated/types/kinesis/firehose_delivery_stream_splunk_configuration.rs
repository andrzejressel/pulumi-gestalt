#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamSplunkConfiguration {
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
                "hec_acknowledgment_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hec_acknowledgment_timeout,
                )
                .await,
            );
            map.insert(
                "hec_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hec_endpoint,
                )
                .await,
            );
            map.insert(
                "hec_endpoint_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hec_endpoint_type,
                )
                .await,
            );
            map.insert(
                "hec_token".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hec_token,
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
                "secrets_manager_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets_manager_configuration,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamSplunkConfiguration {
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
                    r#hec_acknowledgment_timeout: {
                        let field_value = match fields_map.get("hec_acknowledgment_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'hec_acknowledgment_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hec_endpoint: {
                        let field_value = match fields_map.get("hec_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'hec_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hec_endpoint_type: {
                        let field_value = match fields_map.get("hec_endpoint_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'hec_endpoint_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hec_token: {
                        let field_value = match fields_map.get("hec_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'hec_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#secrets_manager_configuration: {
                        let field_value = match fields_map.get("secrets_manager_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets_manager_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
