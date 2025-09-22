#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterDiscoveryEndpoint {
    /// Output only. The IP allocated on the consumer network for the PSC forwarding rule.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// Output only. The port number of the exposed Redis endpoint.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Output only. Customer configuration for where the endpoint
    /// is created and accessed from.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pscConfig")]
    pub r#psc_config: Option<Box<super::super::types::redis::ClusterDiscoveryEndpointPscConfig>>,
}
