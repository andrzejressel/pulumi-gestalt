#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamKinesisSourceConfiguration {
    /// The kinesis stream used as the source of the firehose delivery stream.
    #[builder(into)]
    #[serde(rename = "kinesisStreamArn")]
    pub r#kinesis_stream_arn: String,
    /// The ARN of the role that provides access to the source Kinesis stream.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
}
