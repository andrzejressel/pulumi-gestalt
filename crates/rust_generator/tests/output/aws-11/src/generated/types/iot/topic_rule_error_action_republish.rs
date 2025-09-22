#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleErrorActionRepublish {
    /// The Quality of Service (QoS) level to use when republishing messages. Valid values are 0 or 1. The default value is 0.
    #[builder(into)]
    #[serde(rename = "qos")]
    pub r#qos: Option<i32>,
    /// The ARN of the IAM role that grants access.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The name of the MQTT topic the message should be republished to.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: String,
}
