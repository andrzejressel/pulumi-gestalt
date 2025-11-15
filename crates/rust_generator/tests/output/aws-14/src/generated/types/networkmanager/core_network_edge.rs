#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CoreNetworkEdge {
    /// ASN of a core network edge.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: Option<i32>,
    /// Region where a core network edge is located.
    #[builder(into)]
    #[serde(rename = "edgeLocation")]
    pub r#edge_location: Option<String>,
    /// Inside IP addresses used for core network edges.
    #[builder(into)]
    #[serde(rename = "insideCidrBlocks")]
    pub r#inside_cidr_blocks: Option<Vec<String>>,
}
