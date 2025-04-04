#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodePoolNodeConfigSandboxConfig {
    /// Which sandbox to use for pods in the node pool.
    /// Accepted values are:
    /// 
    /// * `"gvisor"`: Pods run within a gVisor sandbox.
    #[builder(into)]
    #[serde(rename = "sandboxType")]
    pub r#sandbox_type: Box<String>,
}
