#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentSegmentActionViaWithEdgeOverride {
    /// A list of a list of strings. The list of edges associated with the network function group.
    #[builder(into)]
    #[serde(rename = "edgeSets")]
    pub r#edge_sets: Option<Vec<Vec<String>>>,
    /// The preferred edge to use.
    #[builder(into)]
    #[serde(rename = "useEdge")]
    pub r#use_edge: Option<String>,
    /// The preferred edge to use.
    #[builder(into)]
    #[serde(rename = "useEdgeLocation")]
    pub r#use_edge_location: Option<String>,
}
