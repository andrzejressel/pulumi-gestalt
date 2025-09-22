#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallNetworkRuleCollectionRule {
    /// Specifies a description for the rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Either a list of destination IP addresses and/or IP ranges, or a list of destination [Service Tags](https://docs.microsoft.com/azure/virtual-network/service-tags-overview#available-service-tags).
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Option<Vec<String>>,
    /// A list of destination FQDNS for the rule.
    /// 
    /// > **NOTE** [You must enable DNS Proxy to use FQDNs in your network rules](https://docs.microsoft.com/azure/firewall/fqdn-filtering-network-rules).
    /// 
    /// > **NOTE** At least one of `destination_addresses`, `destination_ip_groups` and `destination_fqdns` must be specified for a rule.
    #[builder(into)]
    #[serde(rename = "destinationFqdns")]
    pub r#destination_fqdns: Option<Vec<String>>,
    /// A list of destination IP Group IDs for the rule.
    #[builder(into)]
    #[serde(rename = "destinationIpGroups")]
    pub r#destination_ip_groups: Option<Vec<String>>,
    /// A list of destination ports.
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Vec<String>,
    /// Specifies the name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A list of protocols. Possible values are `Any`, `ICMP`, `TCP` and `UDP`.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Vec<String>,
    /// A list of source IP addresses and/or IP ranges.
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    /// A list of IP Group IDs for the rule.
    /// 
    /// > **NOTE** At least one of `source_addresses` and `source_ip_groups` must be specified for a rule.
    #[builder(into)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Option<Vec<String>>,
}
