#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigAuxiliaryNodeGroupNodeGroup {
    /// The Node group resource name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The node group instance group configuration.
    #[builder(into)]
    #[serde(rename = "nodeGroupConfig")]
    pub r#node_group_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfig>>,
    /// Node group roles. 
    /// One of `"DRIVER"`.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Vec<String>,
}
