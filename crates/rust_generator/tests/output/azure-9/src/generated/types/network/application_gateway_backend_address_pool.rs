#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayBackendAddressPool {
    /// A list of FQDN's which should be part of the Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "fqdns")]
    pub r#fqdns: Option<Vec<String>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// A list of IP Addresses which should be part of the Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Option<Vec<String>>,
    /// The name of the Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
