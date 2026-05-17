#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationElement {
    /// Configuration for Amazon Transcribe Call Analytics processor.
    #[builder(into)]
    #[serde(rename = "amazonTranscribeCallAnalyticsProcessorConfiguration")]
    pub r#amazon_transcribe_call_analytics_processor_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration>>,
    /// Configuration for Amazon Transcribe processor.
    #[builder(into)]
    #[serde(rename = "amazonTranscribeProcessorConfiguration")]
    pub r#amazon_transcribe_processor_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration>>,
    /// Configuration for Kinesis Data Stream sink.
    #[builder(into)]
    #[serde(rename = "kinesisDataStreamSinkConfiguration")]
    pub r#kinesis_data_stream_sink_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration>>,
    /// Configuration for Lambda Function sink.
    #[builder(into)]
    #[serde(rename = "lambdaFunctionSinkConfiguration")]
    pub r#lambda_function_sink_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration>>,
    /// Configuration for S3 recording sink.
    #[builder(into)]
    #[serde(rename = "s3RecordingSinkConfiguration")]
    pub r#s_3_recording_sink_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration>>,
    /// Configuration for SNS Topic sink.
    #[builder(into)]
    #[serde(rename = "snsTopicSinkConfiguration")]
    pub r#sns_topic_sink_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration>>,
    /// Configuration for SQS Queue sink.
    #[builder(into)]
    #[serde(rename = "sqsQueueSinkConfiguration")]
    pub r#sqs_queue_sink_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration>>,
    /// Element type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Configuration for Voice analytics processor.
    #[builder(into)]
    #[serde(rename = "voiceAnalyticsProcessorConfiguration")]
    pub r#voice_analytics_processor_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MediaInsightsPipelineConfigurationElement {
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
                "amazon_transcribe_call_analytics_processor_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#amazon_transcribe_call_analytics_processor_configuration,
                )
                .await,
            );
            map.insert(
                "amazon_transcribe_processor_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#amazon_transcribe_processor_configuration,
                )
                .await,
            );
            map.insert(
                "kinesis_data_stream_sink_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_data_stream_sink_configuration,
                )
                .await,
            );
            map.insert(
                "lambda_function_sink_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lambda_function_sink_configuration,
                )
                .await,
            );
            map.insert(
                "s_3_recording_sink_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_recording_sink_configuration,
                )
                .await,
            );
            map.insert(
                "sns_topic_sink_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sns_topic_sink_configuration,
                )
                .await,
            );
            map.insert(
                "sqs_queue_sink_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sqs_queue_sink_configuration,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );
            map.insert(
                "voice_analytics_processor_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#voice_analytics_processor_configuration,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MediaInsightsPipelineConfigurationElement {
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
                    r#amazon_transcribe_call_analytics_processor_configuration: {
                        let field_value = match fields_map.get("amazon_transcribe_call_analytics_processor_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'amazon_transcribe_call_analytics_processor_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#amazon_transcribe_processor_configuration: {
                        let field_value = match fields_map.get("amazon_transcribe_processor_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'amazon_transcribe_processor_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_data_stream_sink_configuration: {
                        let field_value = match fields_map.get("kinesis_data_stream_sink_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_data_stream_sink_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda_function_sink_configuration: {
                        let field_value = match fields_map.get("lambda_function_sink_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_function_sink_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_recording_sink_configuration: {
                        let field_value = match fields_map.get("s_3_recording_sink_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_recording_sink_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sns_topic_sink_configuration: {
                        let field_value = match fields_map.get("sns_topic_sink_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'sns_topic_sink_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sqs_queue_sink_configuration: {
                        let field_value = match fields_map.get("sqs_queue_sink_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqs_queue_sink_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#voice_analytics_processor_configuration: {
                        let field_value = match fields_map.get("voice_analytics_processor_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'voice_analytics_processor_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
