#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRouterBgpAdvertisedIpRange {
    /// User-specified description for the IP range.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The IP range to advertise. The value must be a
    /// CIDR-formatted string.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: String,
}
