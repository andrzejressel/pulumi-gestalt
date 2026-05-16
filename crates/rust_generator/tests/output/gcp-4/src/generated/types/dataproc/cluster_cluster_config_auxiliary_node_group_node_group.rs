#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
