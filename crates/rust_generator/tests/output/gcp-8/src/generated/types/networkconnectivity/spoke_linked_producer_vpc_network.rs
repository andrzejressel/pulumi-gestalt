#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpokeLinkedProducerVpcNetwork {
    /// IP ranges encompassing the subnets to be excluded from peering.
    #[builder(into)]
    #[serde(rename = "excludeExportRanges")]
    pub r#exclude_export_ranges: Option<Vec<String>>,
    /// IP ranges allowed to be included from peering.
    #[builder(into)]
    #[serde(rename = "includeExportRanges")]
    pub r#include_export_ranges: Option<Vec<String>>,
    /// The URI of the Service Consumer VPC that the Producer VPC is peered with.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    /// The name of the VPC peering between the Service Consumer VPC and the Producer VPC (defined in the Tenant project) which is added to the NCC hub. This peering must be in ACTIVE state.
    #[builder(into)]
    #[serde(rename = "peering")]
    pub r#peering: String,
    /// (Output)
    /// The URI of the Producer VPC.
    #[builder(into)]
    #[serde(rename = "producerNetwork")]
    pub r#producer_network: Option<String>,
}
