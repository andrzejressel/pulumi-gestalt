#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NextGenerationFirewallVirtualNetworkLocalRulestackDestinationNatFrontendConfig {
    /// The port on which to receive traffic.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The ID of the Public IP Address on which to receive traffic. 
    /// 
    /// > **Note:** This must be an Azure Public IP address ID also specified in the `public_ip_address_ids` list.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<String>,
}
