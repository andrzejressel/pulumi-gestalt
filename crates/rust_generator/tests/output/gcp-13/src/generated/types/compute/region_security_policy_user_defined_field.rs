#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionSecurityPolicyUserDefinedField {
    /// The base relative to which 'offset' is measured. Possible values are:
    /// - IPV4: Points to the beginning of the IPv4 header.
    /// - IPV6: Points to the beginning of the IPv6 header.
    /// - TCP: Points to the beginning of the TCP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments.
    /// - UDP: Points to the beginning of the UDP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments.
    /// Possible values are: `IPV4`, `IPV6`, `TCP`, `UDP`.
    #[builder(into)]
    #[serde(rename = "base")]
    pub r#base: String,
    /// If specified, apply this mask (bitwise AND) to the field to ignore bits before matching.
    /// Encoded as a hexadecimal number (starting with "0x").
    /// The last byte of the field (in network byte order) corresponds to the least significant byte of the mask.
    #[builder(into)]
    #[serde(rename = "mask")]
    pub r#mask: Option<String>,
    /// Name of the user-defined field, as given in the definition.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Offset of the first byte of the field (in network byte order) relative to 'base'.
    #[builder(into)]
    #[serde(rename = "offset")]
    pub r#offset: Option<i32>,
    /// Size of the field in bytes. Valid values: 1-4.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<i32>,
}
