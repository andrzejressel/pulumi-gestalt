#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterGcpConfigAccessConfig {
    /// Virtual Private Cloud (VPC) subnets where IP addresses for the Kafka cluster are allocated. To make the cluster available in a VPC, you must specify at least one subnet per network. You must specify between 1 and 10 subnets. Additional subnets may be specified with additional `network_configs` blocks.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "networkConfigs")]
    pub r#network_configs: Vec<super::super::types::managedkafka::ClusterGcpConfigAccessConfigNetworkConfig>,
}
