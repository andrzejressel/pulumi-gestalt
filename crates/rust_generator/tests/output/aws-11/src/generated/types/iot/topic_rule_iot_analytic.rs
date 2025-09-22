#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleIotAnalytic {
    /// The payload that contains a JSON array of records will be sent to IoT Analytics via a batch call.
    #[builder(into)]
    #[serde(rename = "batchMode")]
    pub r#batch_mode: Option<bool>,
    /// Name of AWS IOT Analytics channel.
    #[builder(into)]
    #[serde(rename = "channelName")]
    pub r#channel_name: String,
    /// The ARN of the IAM role that grants access.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
}
