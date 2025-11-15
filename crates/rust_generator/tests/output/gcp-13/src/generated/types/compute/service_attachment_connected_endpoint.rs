#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceAttachmentConnectedEndpoint {
    /// (Output)
    /// The url of the consumer network.
    #[builder(into)]
    #[serde(rename = "consumerNetwork")]
    pub r#consumer_network: Option<String>,
    /// (Output)
    /// The URL of the consumer forwarding rule.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Option<String>,
    /// (Output)
    /// The number of consumer Network Connectivity Center spokes that the connected Private Service Connect endpoint has propagated to.
    #[builder(into)]
    #[serde(rename = "propagatedConnectionCount")]
    pub r#propagated_connection_count: Option<i32>,
    /// (Output)
    /// The PSC connection id of the connected endpoint.
    #[builder(into)]
    #[serde(rename = "pscConnectionId")]
    pub r#psc_connection_id: Option<String>,
    /// (Output)
    /// The status of the connection from the consumer forwarding rule to
    /// this service attachment.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
