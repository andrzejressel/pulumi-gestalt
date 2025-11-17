#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpokeLinkedVpcNetwork {
    /// IP ranges encompassing the subnets to be excluded from peering.
    #[builder(into)]
    #[serde(rename = "excludeExportRanges")]
    pub r#exclude_export_ranges: Option<Vec<String>>,
    /// IP ranges allowed to be included from peering.
    #[builder(into)]
    #[serde(rename = "includeExportRanges")]
    pub r#include_export_ranges: Option<Vec<String>>,
    /// The URI of the VPC network resource.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
