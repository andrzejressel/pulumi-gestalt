#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionEventingRuntimeData {
    /// Events listener endpoint. The value will populated after provisioning the events listener.
    #[builder(into)]
    #[serde(rename = "eventsListenerEndpoint")]
    pub r#events_listener_endpoint: Option<String>,
    /// (Output)
    /// Current status of eventing.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "statuses")]
    pub r#statuses: Option<Vec<super::super::types::integrationconnectors::ConnectionEventingRuntimeDataStatus>>,
}
