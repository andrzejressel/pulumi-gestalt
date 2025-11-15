#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDomainInputMappingField {
    /// Specifies the default data version of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "dataVersion")]
    pub r#data_version: String,
    /// Specifies the event time of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "eventTime")]
    pub r#event_time: String,
    /// Specifies the default event type of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: String,
    /// Specifies the id of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Specifies the default subject of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: String,
    /// Specifies the topic of the EventGrid Event associated with the domain.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: String,
}
