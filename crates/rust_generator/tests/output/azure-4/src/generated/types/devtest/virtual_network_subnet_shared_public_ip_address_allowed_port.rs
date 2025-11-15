#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNetworkSubnetSharedPublicIpAddressAllowedPort {
    /// The port on the Virtual Machine that the traffic will be sent to.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: Option<i32>,
    /// The transport protocol that the traffic will use. Possible values are `TCP` and `UDP`.
    #[builder(into)]
    #[serde(rename = "transportProtocol")]
    pub r#transport_protocol: Option<String>,
}
