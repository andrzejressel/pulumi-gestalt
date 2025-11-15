#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayBackendAddressPool {
    /// A list of FQDNs which are part of the Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "fqdns")]
    pub r#fqdns: Vec<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// A list of IP Addresses which are part of the Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Vec<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
