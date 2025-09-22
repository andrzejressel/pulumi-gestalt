#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MediaInsightsPipelineConfigurationElement {
    /// Configuration for Amazon Transcribe Call Analytics processor.
    #[builder(into)]
    #[serde(rename = "amazonTranscribeCallAnalyticsProcessorConfiguration")]
    pub r#amazon_transcribe_call_analytics_processor_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration>>,
    /// Configuration for Amazon Transcribe processor.
    #[builder(into)]
    #[serde(rename = "amazonTranscribeProcessorConfiguration")]
    pub r#amazon_transcribe_processor_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration>>,
    /// Configuration for Kinesis Data Stream sink.
    #[builder(into)]
    #[serde(rename = "kinesisDataStreamSinkConfiguration")]
    pub r#kinesis_data_stream_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration>>,
    /// Configuration for Lambda Function sink.
    #[builder(into)]
    #[serde(rename = "lambdaFunctionSinkConfiguration")]
    pub r#lambda_function_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration>>,
    /// Configuration for S3 recording sink.
    #[builder(into)]
    #[serde(rename = "s3RecordingSinkConfiguration")]
    pub r#s_3_recording_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration>>,
    /// Configuration for SNS Topic sink.
    #[builder(into)]
    #[serde(rename = "snsTopicSinkConfiguration")]
    pub r#sns_topic_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration>>,
    /// Configuration for SQS Queue sink.
    #[builder(into)]
    #[serde(rename = "sqsQueueSinkConfiguration")]
    pub r#sqs_queue_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration>>,
    /// Element type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Configuration for Voice analytics processor.
    #[builder(into)]
    #[serde(rename = "voiceAnalyticsProcessorConfiguration")]
    pub r#voice_analytics_processor_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration>>,
}
