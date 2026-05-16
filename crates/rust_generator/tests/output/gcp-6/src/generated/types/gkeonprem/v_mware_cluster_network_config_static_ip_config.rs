#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterNetworkConfigStaticIpConfig {
    /// Represents the configuration values for static IP allocation to nodes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ipBlocks")]
    pub r#ip_blocks: Vec<super::super::types::gkeonprem::VMwareClusterNetworkConfigStaticIpConfigIpBlock>,
}
