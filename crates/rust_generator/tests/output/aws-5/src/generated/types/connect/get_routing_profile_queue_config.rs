#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRoutingProfileQueueConfig {
    /// Channels agents can handle in the Contact Control Panel (CCP) for this routing profile. Valid values are `VOICE`, `CHAT`, `TASK`.
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: String,
    /// Delay, in seconds, that a contact should be in the queue before they are routed to an available agent
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: i32,
    /// Order in which contacts are to be handled for the queue.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// ARN for the queue.
    #[builder(into)]
    #[serde(rename = "queueArn")]
    pub r#queue_arn: String,
    /// Identifier for the queue.
    #[builder(into)]
    #[serde(rename = "queueId")]
    pub r#queue_id: String,
    /// Name for the queue.
    #[builder(into)]
    #[serde(rename = "queueName")]
    pub r#queue_name: String,
}
