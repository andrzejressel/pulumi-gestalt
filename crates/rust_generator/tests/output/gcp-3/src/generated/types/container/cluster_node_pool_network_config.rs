#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodePoolNetworkConfig {
    /// We specify the additional node networks for this node pool using this list. Each node network corresponds to an additional interface
    #[builder(into)]
    #[serde(rename = "additionalNodeNetworkConfigs")]
    pub r#additional_node_network_configs: Option<Vec<super::super::types::container::ClusterNodePoolNetworkConfigAdditionalNodeNetworkConfig>>,
    /// We specify the additional pod networks for this node pool using this list. Each pod network corresponds to an additional alias IP range for the node
    #[builder(into)]
    #[serde(rename = "additionalPodNetworkConfigs")]
    pub r#additional_pod_network_configs: Option<Vec<super::super::types::container::ClusterNodePoolNetworkConfigAdditionalPodNetworkConfig>>,
    /// Whether to create a new range for pod IPs in this node pool. Defaults are provided for `pod_range` and `pod_ipv4_cidr_block` if they are not specified.
    #[builder(into)]
    #[serde(rename = "createPodRange")]
    pub r#create_pod_range: Option<bool>,
    /// Whether nodes have internal IP addresses only.
    #[builder(into)]
    #[serde(rename = "enablePrivateNodes")]
    pub r#enable_private_nodes: Option<bool>,
    /// Network bandwidth tier configuration.
    #[builder(into)]
    #[serde(rename = "networkPerformanceConfig")]
    pub r#network_performance_config: Box<Option<super::super::types::container::ClusterNodePoolNetworkConfigNetworkPerformanceConfig>>,
    /// Configuration for node-pool level pod cidr overprovision. If not set, the cluster level setting will be inherited
    #[builder(into)]
    #[serde(rename = "podCidrOverprovisionConfig")]
    pub r#pod_cidr_overprovision_config: Box<Option<super::super::types::container::ClusterNodePoolNetworkConfigPodCidrOverprovisionConfig>>,
    /// The IP address range for pod IPs in this node pool. Only applicable if createPodRange is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. /14) to have a range chosen with a specific netmask. Set to a CIDR notation (e.g. 10.96.0.0/14) to pick a specific range to use.
    #[builder(into)]
    #[serde(rename = "podIpv4CidrBlock")]
    pub r#pod_ipv_4_cidr_block: Option<String>,
    /// The ID of the secondary range for pod IPs. If `create_pod_range` is true, this ID is used for the new range. If `create_pod_range` is false, uses an existing secondary range with this ID.
    #[builder(into)]
    #[serde(rename = "podRange")]
    pub r#pod_range: Option<String>,
}
