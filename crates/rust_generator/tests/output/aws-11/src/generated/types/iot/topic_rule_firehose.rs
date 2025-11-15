#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleFirehose {
    /// The payload that contains a JSON array of records will be sent to Kinesis Firehose via a batch call.
    #[builder(into)]
    #[serde(rename = "batchMode")]
    pub r#batch_mode: Option<bool>,
    /// The delivery stream name.
    #[builder(into)]
    #[serde(rename = "deliveryStreamName")]
    pub r#delivery_stream_name: String,
    /// The IAM role ARN that grants access to the Amazon Kinesis Firehose stream.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// A character separator that is used to separate records written to the Firehose stream. Valid values are: '\n' (newline), '\t' (tab), '\r\n' (Windows newline), ',' (comma).
    #[builder(into)]
    #[serde(rename = "separator")]
    pub r#separator: Option<String>,
}
