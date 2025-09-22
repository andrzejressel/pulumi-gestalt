#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HadoopClusterRolesEdgeNodeHttpsEndpoint {
    /// A list of access modes for the application.
    #[builder(into)]
    #[serde(rename = "accessModes")]
    pub r#access_modes: Option<Vec<String>>,
    /// The destination port to connect to.
    #[builder(into)]
    #[serde(rename = "destinationPort")]
    pub r#destination_port: Option<i32>,
    /// The value indicates whether the gateway authentication is enabled or not.
    #[builder(into)]
    #[serde(rename = "disableGatewayAuth")]
    pub r#disable_gateway_auth: Option<bool>,
    /// The private ip address of the endpoint.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The application's subdomain suffix.
    #[builder(into)]
    #[serde(rename = "subDomainSuffix")]
    pub r#sub_domain_suffix: Option<String>,
}
