#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentNodeConfig {
    /// (Output)
    /// The current total number of gateway nodes that each environment currently has across
    /// all instances.
    #[builder(into)]
    #[serde(rename = "currentAggregateNodeCount")]
    pub r#current_aggregate_node_count: Option<String>,
    /// The maximum total number of gateway nodes that the is reserved for all instances that
    /// has the specified environment. If not specified, the default is determined by the
    /// recommended maximum number of nodes for that gateway.
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Option<String>,
    /// The minimum total number of gateway nodes that the is reserved for all instances that
    /// has the specified environment. If not specified, the default is determined by the
    /// recommended minimum number of nodes for that gateway.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Option<String>,
}
