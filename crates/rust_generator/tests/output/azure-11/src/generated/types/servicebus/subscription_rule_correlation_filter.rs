#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SubscriptionRuleCorrelationFilter {
    /// Content type of the message.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    /// Identifier of the correlation.
    #[builder(into)]
    #[serde(rename = "correlationId")]
    pub r#correlation_id: Option<String>,
    /// Application specific label.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// Identifier of the message.
    #[builder(into)]
    #[serde(rename = "messageId")]
    pub r#message_id: Option<String>,
    /// A list of user defined properties to be included in the filter. Specified as a map of name/value pairs.
    /// 
    /// > **NOTE:** When creating a subscription rule of type `CorrelationFilter` at least one property must be set in the `correlation_filter` block.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// Address of the queue to reply to.
    #[builder(into)]
    #[serde(rename = "replyTo")]
    pub r#reply_to: Option<String>,
    /// Session identifier to reply to.
    #[builder(into)]
    #[serde(rename = "replyToSessionId")]
    pub r#reply_to_session_id: Option<String>,
    /// Session identifier.
    #[builder(into)]
    #[serde(rename = "sessionId")]
    pub r#session_id: Option<String>,
    /// Address to send to.
    #[builder(into)]
    #[serde(rename = "to")]
    pub r#to: Option<String>,
}
