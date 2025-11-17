#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventGridTopicInputMappingDefaultValues {
    /// Specifies the default data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "dataVersion")]
    pub r#data_version: Option<String>,
    /// Specifies the default event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Option<String>,
    /// Specifies the default subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
}
