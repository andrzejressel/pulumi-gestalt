#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterNodePoolLinuxOsConfig {
    /// Specifies the size of swap file on each node in MB. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "swapFileSizeMb")]
    pub r#swap_file_size_mb: Option<i32>,
    /// A `sysctl_config` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sysctlConfig")]
    pub r#sysctl_config: Option<Box<super::super::types::containerservice::KubernetesClusterNodePoolLinuxOsConfigSysctlConfig>>,
    /// specifies the defrag configuration for Transparent Huge Page. Possible values are `always`, `defer`, `defer+madvise`, `madvise` and `never`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "transparentHugePageDefrag")]
    pub r#transparent_huge_page_defrag: Option<String>,
    /// Specifies the Transparent Huge Page enabled configuration. Possible values are `always`, `madvise` and `never`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "transparentHugePageEnabled")]
    pub r#transparent_huge_page_enabled: Option<String>,
}
