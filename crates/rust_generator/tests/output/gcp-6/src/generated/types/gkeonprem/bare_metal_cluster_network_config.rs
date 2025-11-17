#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterNetworkConfig {
    /// Enables the use of advanced Anthos networking features, such as Bundled
    /// Load Balancing with BGP or the egress NAT gateway.
    /// Setting configuration for advanced networking features will automatically
    /// set this flag.
    #[builder(into)]
    #[serde(rename = "advancedNetworking")]
    pub r#advanced_networking: Option<bool>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "islandModeCidr")]
    pub r#island_mode_cidr: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigIslandModeCidr>>,
    /// Configuration for multiple network interfaces.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "multipleNetworkInterfacesConfig")]
    pub r#multiple_network_interfaces_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigMultipleNetworkInterfacesConfig>>,
    /// Configuration for SR-IOV.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "srIovConfig")]
    pub r#sr_iov_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterNetworkConfigSrIovConfig>>,
}
