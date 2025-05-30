#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodePoolNodeConfigLinuxNodeConfig {
    /// cgroupMode specifies the cgroup mode to be used on the node.
    #[builder(into)]
    #[serde(rename = "cgroupMode")]
    pub r#cgroup_mode: Box<String>,
    /// Amounts for 2M and 1G hugepages.
    #[builder(into)]
    #[serde(rename = "hugepagesConfigs")]
    pub r#hugepages_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigLinuxNodeConfigHugepagesConfig>>,
    /// The Linux kernel parameters to be applied to the nodes and all pods running on the nodes.
    #[builder(into)]
    #[serde(rename = "sysctls")]
    pub r#sysctls: Box<std::collections::HashMap<String, String>>,
}
