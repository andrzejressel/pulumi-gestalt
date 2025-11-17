#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceDiscoveryEndpoint {
    /// (Output)
    /// Output only. IP address of the exposed endpoint clients connect to.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// (Output)
    /// Output only. The consumer network where the IP address resides, in the form of
    /// projects/{project_id}/global/networks/{network_id}.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// (Output)
    /// Output only. Ports of the exposed endpoint.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}
