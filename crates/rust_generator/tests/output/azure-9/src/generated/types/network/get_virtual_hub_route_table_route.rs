#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualHubRouteTableRoute {
    /// A list of destination addresses for this route.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Vec<String>,
    /// The type of destinations.
    #[builder(into)]
    #[serde(rename = "destinationsType")]
    pub r#destinations_type: String,
    /// The name of the Virtual Hub Route Table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The next hop's resource ID.
    #[builder(into)]
    #[serde(rename = "nextHop")]
    pub r#next_hop: String,
    /// The type of next hop.
    #[builder(into)]
    #[serde(rename = "nextHopType")]
    pub r#next_hop_type: String,
}
