#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CoreNetworkSegment {
    /// Regions where the edges are located.
    #[builder(into)]
    #[serde(rename = "edgeLocations")]
    pub r#edge_locations: Option<Vec<String>>,
    /// Name of a core network segment.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Shared segments of a core network.
    #[builder(into)]
    #[serde(rename = "sharedSegments")]
    pub r#shared_segments: Option<Vec<String>>,
}
