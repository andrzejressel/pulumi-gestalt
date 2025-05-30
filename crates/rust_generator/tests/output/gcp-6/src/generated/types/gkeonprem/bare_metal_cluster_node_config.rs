#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterNodeConfig {
    /// The available runtimes that can be used to run containers in a Bare Metal User Cluster.
    /// Possible values are: `CONTAINER_RUNTIME_UNSPECIFIED`, `DOCKER`, `CONTAINERD`.
    #[builder(into, default)]
    #[serde(rename = "containerRuntime")]
    pub r#container_runtime: Box<Option<String>>,
    /// The maximum number of pods a node can run. The size of the CIDR range
    /// assigned to the node will be derived from this parameter.
    #[builder(into, default)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: Box<Option<i32>>,
}
