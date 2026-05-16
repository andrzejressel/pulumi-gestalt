#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfig {
    /// The autoscaler configuration for this node pool. 
    /// The autoscaler is enabled only when a valid configuration is present.
    #[builder(into)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Option<Box<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigAutoscaling>>,
    /// The node pool configuration.
    #[builder(into)]
    #[serde(rename = "config")]
    pub r#config: Option<Box<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigConfig>>,
    /// The list of Compute Engine zones where node pool nodes associated 
    /// with a Dataproc on GKE virtual cluster will be located.
    /// - - -
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Vec<String>,
}
