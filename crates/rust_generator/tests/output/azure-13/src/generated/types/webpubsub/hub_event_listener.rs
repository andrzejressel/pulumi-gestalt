#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HubEventListener {
    /// Specifies the event hub name to receive the events.
    #[builder(into)]
    #[serde(rename = "eventhubName")]
    pub r#eventhub_name: String,
    /// Specifies the event hub namespace name to receive the events.
    #[builder(into)]
    #[serde(rename = "eventhubNamespaceName")]
    pub r#eventhub_namespace_name: String,
    /// Specifies the list of system events. Supported values are `connected` and `disconnected`.
    #[builder(into)]
    #[serde(rename = "systemEventNameFilters")]
    pub r#system_event_name_filters: Option<Vec<String>>,
    /// Specifies the list of matching user event names. `["*"]` can be used to match all events.
    #[builder(into)]
    #[serde(rename = "userEventNameFilters")]
    pub r#user_event_name_filters: Option<Vec<String>>,
}
