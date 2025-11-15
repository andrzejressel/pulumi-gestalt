#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetNetworkProfile {
    /// Specifies whether to enable accelerated networking or not.
    #[builder(into)]
    #[serde(rename = "acceleratedNetworking")]
    pub r#accelerated_networking: Option<bool>,
    /// A `dns_settings` block as documented below.
    #[builder(into)]
    #[serde(rename = "dnsSettings")]
    pub r#dns_settings: Option<Box<super::super::types::compute::ScaleSetNetworkProfileDnsSettings>>,
    /// An `ip_configuration` block as documented below.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Vec<super::super::types::compute::ScaleSetNetworkProfileIpConfiguration>,
    /// Whether IP forwarding is enabled on this NIC. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ipForwarding")]
    pub r#ip_forwarding: Option<bool>,
    /// Specifies the name of the network interface configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the identifier for the network security group.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: Option<String>,
    /// Indicates whether network interfaces created from the network interface configuration will be the primary NIC of the VM.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
}
