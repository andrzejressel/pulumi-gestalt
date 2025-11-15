#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceEndpointConnectionsPrivateEndpointConnection {
    /// A message indicating if changes on the service provider require any updates or not.
    #[builder(into)]
    #[serde(rename = "actionRequired")]
    pub r#action_required: String,
    /// The resource id of the private link service connection between the private link service and the private link endpoint.
    #[builder(into)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: String,
    /// The name of the connection between the private link service and the private link endpoint.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: String,
    /// The request for approval message or the reason for rejection message.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The resource id of the private link endpoint.
    #[builder(into)]
    #[serde(rename = "privateEndpointId")]
    pub r#private_endpoint_id: String,
    /// The name of the private link endpoint.
    #[builder(into)]
    #[serde(rename = "privateEndpointName")]
    pub r#private_endpoint_name: String,
    /// Indicates the state of the connection between the private link service and the private link endpoint, possible values are `Pending`, `Approved` or `Rejected`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}
