#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalAdminClusterControlPlaneControlPlaneNodePoolConfigNodePoolConfig {
    /// The map of Kubernetes labels (key/value pairs) to be applied to
    /// each node. These will added in addition to any default label(s)
    /// that Kubernetes may apply to the node. In case of conflict in
    /// label keys, the applied set may differ depending on the Kubernetes
    /// version -- it's best to assume the behavior is undefined and
    /// conflicts should be avoided. For more information, including usage
    /// and the valid values, see:
    /// - http://kubernetes.io/v1.1/docs/user-guide/labels.html
    /// An object containing a list of "key": value pairs.
    /// For example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// The list of machine addresses in the Bare Metal Node Pool.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nodeConfigs")]
    pub r#node_configs: Option<Vec<super::super::types::gkeonprem::BareMetalAdminClusterControlPlaneControlPlaneNodePoolConfigNodePoolConfigNodeConfig>>,
    /// Specifies the nodes operating system (default: LINUX).
    #[builder(into)]
    #[serde(rename = "operatingSystem")]
    pub r#operating_system: Option<String>,
    /// The initial taints assigned to nodes of this node pool.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "taints")]
    pub r#taints: Option<Vec<super::super::types::gkeonprem::BareMetalAdminClusterControlPlaneControlPlaneNodePoolConfigNodePoolConfigTaint>>,
}
