#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterRemoteNetworkConfig {
    /// The networks that can contain hybrid nodes.
    #[builder(into)]
    #[serde(rename = "remoteNodeNetworks")]
    pub r#remote_node_networks: Vec<super::super::types::eks::GetClusterRemoteNetworkConfigRemoteNodeNetwork>,
    /// The networks that can contain pods that run Kubernetes webhooks on hybrid nodes.
    #[builder(into)]
    #[serde(rename = "remotePodNetworks")]
    pub r#remote_pod_networks: Vec<super::super::types::eks::GetClusterRemoteNetworkConfigRemotePodNetwork>,
}
