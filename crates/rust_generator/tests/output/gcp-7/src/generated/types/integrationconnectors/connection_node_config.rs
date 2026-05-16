#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionNodeConfig {
    /// Minimum number of nodes in the runtime nodes.
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Option<i32>,
    /// Minimum number of nodes in the runtime nodes.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Option<i32>,
}
