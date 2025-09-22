#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionSecurityPolicyRuleNetworkMatch {
    /// Destination IPv4/IPv6 addresses or CIDR prefixes, in standard text format.
    #[builder(into)]
    #[serde(rename = "destIpRanges")]
    pub r#dest_ip_ranges: Option<Vec<String>>,
    /// Destination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. "80") or range (e.g. "0-1023").
    #[builder(into)]
    #[serde(rename = "destPorts")]
    pub r#dest_ports: Option<Vec<String>>,
    /// IPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. "6"), range (e.g. "253-254"), or one of the following protocol names: "tcp", "udp", "icmp", "esp", "ah", "ipip", or "sctp".
    #[builder(into)]
    #[serde(rename = "ipProtocols")]
    pub r#ip_protocols: Option<Vec<String>>,
    /// BGP Autonomous System Number associated with the source IP address.
    #[builder(into)]
    #[serde(rename = "srcAsns")]
    pub r#src_asns: Option<Vec<i32>>,
    /// Source IPv4/IPv6 addresses or CIDR prefixes, in standard text format.
    #[builder(into)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Option<Vec<String>>,
    /// Source port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. "80") or range (e.g. "0-1023").
    #[builder(into)]
    #[serde(rename = "srcPorts")]
    pub r#src_ports: Option<Vec<String>>,
    /// Two-letter ISO 3166-1 alpha-2 country code associated with the source IP address.
    #[builder(into)]
    #[serde(rename = "srcRegionCodes")]
    pub r#src_region_codes: Option<Vec<String>>,
    /// User-defined fields. Each element names a defined field and lists the matching values for that field.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userDefinedFields")]
    pub r#user_defined_fields: Option<Vec<super::super::types::compute::RegionSecurityPolicyRuleNetworkMatchUserDefinedField>>,
}
