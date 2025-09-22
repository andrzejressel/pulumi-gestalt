#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyRuleMatch {
    /// Address groups which should be matched against the traffic destination. Maximum number of destination address groups is 10.
    #[builder(into)]
    #[serde(rename = "destAddressGroups")]
    pub r#dest_address_groups: Option<Vec<String>>,
    /// Fully Qualified Domain Name (FQDN) which should be matched against traffic destination. Maximum number of destination fqdn allowed is 100.
    #[builder(into)]
    #[serde(rename = "destFqdns")]
    pub r#dest_fqdns: Option<Vec<String>>,
    /// CIDR IP address range. Maximum number of destination CIDR IP ranges allowed is 5000.
    #[builder(into)]
    #[serde(rename = "destIpRanges")]
    pub r#dest_ip_ranges: Option<Vec<String>>,
    /// Region codes whose IP addresses will be used to match for destination of traffic. Should be specified as 2 letter country code defined as per ISO 3166 alpha-2 country codes. ex."US" Maximum number of dest region codes allowed is 5000.
    #[builder(into)]
    #[serde(rename = "destRegionCodes")]
    pub r#dest_region_codes: Option<Vec<String>>,
    /// Names of Network Threat Intelligence lists. The IPs in these lists will be matched against traffic destination.
    #[builder(into)]
    #[serde(rename = "destThreatIntelligences")]
    pub r#dest_threat_intelligences: Option<Vec<String>>,
    /// Pairs of IP protocols and ports that the rule should match.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "layer4Configs")]
    pub r#layer_4_configs: Vec<super::super::types::compute::FirewallPolicyRuleMatchLayer4Config>,
    /// Address groups which should be matched against the traffic source. Maximum number of source address groups is 10.
    #[builder(into)]
    #[serde(rename = "srcAddressGroups")]
    pub r#src_address_groups: Option<Vec<String>>,
    /// Fully Qualified Domain Name (FQDN) which should be matched against traffic source. Maximum number of source fqdn allowed is 100.
    #[builder(into)]
    #[serde(rename = "srcFqdns")]
    pub r#src_fqdns: Option<Vec<String>>,
    /// CIDR IP address range. Maximum number of source CIDR IP ranges allowed is 5000.
    #[builder(into)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Option<Vec<String>>,
    /// Region codes whose IP addresses will be used to match for source of traffic. Should be specified as 2 letter country code defined as per ISO 3166 alpha-2 country codes. ex."US" Maximum number of source region codes allowed is 5000.
    #[builder(into)]
    #[serde(rename = "srcRegionCodes")]
    pub r#src_region_codes: Option<Vec<String>>,
    /// Names of Network Threat Intelligence lists. The IPs in these lists will be matched against traffic source.
    /// 
    /// 
    /// <a name="nested_layer4_configs"></a>The `layer4_configs` block supports:
    #[builder(into)]
    #[serde(rename = "srcThreatIntelligences")]
    pub r#src_threat_intelligences: Option<Vec<String>>,
}
