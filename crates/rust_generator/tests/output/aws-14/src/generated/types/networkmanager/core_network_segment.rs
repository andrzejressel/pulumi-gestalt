#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CoreNetworkSegment {
    /// Regions where the edges are located.
    #[builder(into, default)]
    #[serde(rename = "edgeLocations")]
    pub r#edge_locations: Box<Option<Vec<String>>>,
    /// Name of a core network segment.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Shared segments of a core network.
    #[builder(into, default)]
    #[serde(rename = "sharedSegments")]
    pub r#shared_segments: Box<Option<Vec<String>>>,
}
