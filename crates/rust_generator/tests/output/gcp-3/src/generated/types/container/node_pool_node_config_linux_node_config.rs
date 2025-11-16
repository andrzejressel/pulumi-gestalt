#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodePoolNodeConfigLinuxNodeConfig {
    /// cgroupMode specifies the cgroup mode to be used on the node.
    #[builder(into)]
    #[serde(rename = "cgroupMode")]
    pub r#cgroup_mode: Option<String>,
    /// Amounts for 2M and 1G hugepages.
    #[builder(into)]
    #[serde(rename = "hugepagesConfig")]
    pub r#hugepages_config: Option<Box<super::super::types::container::NodePoolNodeConfigLinuxNodeConfigHugepagesConfig>>,
    /// The Linux kernel parameters to be applied to the nodes and all pods running on the nodes.
    #[builder(into)]
    #[serde(rename = "sysctls")]
    pub r#sysctls: Option<std::collections::HashMap<String, String>>,
}
