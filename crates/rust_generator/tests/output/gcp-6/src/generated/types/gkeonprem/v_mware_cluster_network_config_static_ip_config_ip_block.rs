#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterNetworkConfigStaticIpConfigIpBlock {
    /// The network gateway used by the VMware User Cluster.
    #[builder(into)]
    #[serde(rename = "gateway")]
    pub r#gateway: String,
    /// The node's network configurations used by the VMware User Cluster.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ips")]
    pub r#ips: Vec<super::super::types::gkeonprem::VMwareClusterNetworkConfigStaticIpConfigIpBlockIp>,
    /// The netmask used by the VMware User Cluster.
    #[builder(into)]
    #[serde(rename = "netmask")]
    pub r#netmask: String,
}
