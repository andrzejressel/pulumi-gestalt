#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetQuickConnectQuickConnectConfigQueueConfig {
    /// Identifier of the contact flow.
    #[builder(into)]
    #[serde(rename = "contactFlowId")]
    pub r#contact_flow_id: Box<String>,
    /// Identifier for the queue.
    #[builder(into)]
    #[serde(rename = "queueId")]
    pub r#queue_id: Box<String>,
}
