#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigGkeClusterConfigNamespacedGkeDeploymentTarget {
    /// A namespace within the GKE cluster to deploy into.
    #[builder(into)]
    #[serde(rename = "clusterNamespace")]
    pub r#cluster_namespace: Option<String>,
    /// The target GKE cluster to deploy to. Format: 'projects/{project}/locations/{location}/clusters/{cluster_id}'
    #[builder(into)]
    #[serde(rename = "targetGkeCluster")]
    pub r#target_gke_cluster: Option<String>,
}
