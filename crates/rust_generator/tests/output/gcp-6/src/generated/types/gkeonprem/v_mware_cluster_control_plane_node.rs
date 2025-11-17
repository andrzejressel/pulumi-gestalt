#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterControlPlaneNode {
    /// AutoResizeConfig provides auto resizing configurations.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autoResizeConfig")]
    pub r#auto_resize_config: Option<Box<super::super::types::gkeonprem::VMwareClusterControlPlaneNodeAutoResizeConfig>>,
    /// The number of CPUs for each admin cluster node that serve as control planes
    /// for this VMware User Cluster. (default: 4 CPUs)
    #[builder(into)]
    #[serde(rename = "cpus")]
    pub r#cpus: Option<i32>,
    /// The megabytes of memory for each admin cluster node that serves as a
    /// control plane for this VMware User Cluster (default: 8192 MB memory).
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Option<i32>,
    /// The number of control plane nodes for this VMware User Cluster.
    /// (default: 1 replica).
    #[builder(into)]
    #[serde(rename = "replicas")]
    pub r#replicas: Option<i32>,
    /// (Output)
    /// Vsphere-specific config.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vsphereConfigs")]
    pub r#vsphere_configs: Option<Vec<super::super::types::gkeonprem::VMwareClusterControlPlaneNodeVsphereConfig>>,
}
