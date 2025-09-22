#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodeGroupAutoscalingPolicy {
    /// Maximum size of the node group. Set to a value less than or equal
    /// to 100 and greater than or equal to min-nodes.
    #[builder(into)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: Option<i32>,
    /// Minimum size of the node group. Must be less
    /// than or equal to max-nodes. The default value is 0.
    #[builder(into)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: Option<i32>,
    /// The autoscaling mode. Set to one of the following:
    /// - OFF: Disables the autoscaler.
    /// - ON: Enables scaling in and scaling out.
    /// - ONLY_SCALE_OUT: Enables only scaling out.
    /// You must use this mode if your node groups are configured to
    /// restart their hosted VMs on minimal servers.
    /// Possible values are: `OFF`, `ON`, `ONLY_SCALE_OUT`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
}
