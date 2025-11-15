#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubnetworkSecondaryIpRange {
    /// The range of IP addresses belonging to this subnetwork secondary
    /// range. Provide this property when you create the subnetwork.
    /// Ranges must be unique and non-overlapping with all primary and
    /// secondary IP ranges within a network. Only IPv4 is supported.
    /// Field is optional when `reserved_internal_range` is defined, otherwise required.
    #[builder(into)]
    #[serde(rename = "ipCidrRange")]
    pub r#ip_cidr_range: Option<String>,
    /// The name associated with this subnetwork secondary range, used
    /// when adding an alias IP range to a VM instance. The name must
    /// be 1-63 characters long, and comply with RFC1035. The name
    /// must be unique within the subnetwork.
    #[builder(into)]
    #[serde(rename = "rangeName")]
    pub r#range_name: String,
    /// The ID of the reserved internal range. Must be prefixed with `networkconnectivity.googleapis.com`
    /// E.g. `networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}`
    #[builder(into)]
    #[serde(rename = "reservedInternalRange")]
    pub r#reserved_internal_range: Option<String>,
}
