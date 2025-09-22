#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkSecurityGroupSecurityRule {
    /// Specifies whether network traffic is allowed or denied. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: String,
    /// A description for this rule. Restricted to 140 characters.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// CIDR or destination IP range or * to match any IP. Tags such as `VirtualNetwork`, `AzureLoadBalancer` and `Internet` can also be used. This is required if `destination_address_prefixes` is not specified.
    #[builder(into)]
    #[serde(rename = "destinationAddressPrefix")]
    pub r#destination_address_prefix: Option<String>,
    /// List of destination address prefixes. Tags may not be used. This is required if `destination_address_prefix` is not specified.
    #[builder(into)]
    #[serde(rename = "destinationAddressPrefixes")]
    pub r#destination_address_prefixes: Option<Vec<String>>,
    /// A List of destination Application Security Group IDs
    #[builder(into)]
    #[serde(rename = "destinationApplicationSecurityGroupIds")]
    pub r#destination_application_security_group_ids: Option<Vec<String>>,
    /// Destination Port or Range. Integer or range between `0` and `65535` or `*` to match any. This is required if `destination_port_ranges` is not specified.
    #[builder(into)]
    #[serde(rename = "destinationPortRange")]
    pub r#destination_port_range: Option<String>,
    /// List of destination ports or port ranges. This is required if `destination_port_range` is not specified.
    #[builder(into)]
    #[serde(rename = "destinationPortRanges")]
    pub r#destination_port_ranges: Option<Vec<String>>,
    /// The direction specifies if rule will be evaluated on incoming or outgoing traffic. Possible values are `Inbound` and `Outbound`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: String,
    /// The name of the security rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the priority of the rule. The value can be between 100 and 4096. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// Network protocol this rule applies to. Possible values include `Tcp`, `Udp`, `Icmp`, `Esp`, `Ah` or `*` (which matches all).
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// CIDR or source IP range or * to match any IP. Tags such as `VirtualNetwork`, `AzureLoadBalancer` and `Internet` can also be used. This is required if `source_address_prefixes` is not specified.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefix")]
    pub r#source_address_prefix: Option<String>,
    /// List of source address prefixes. Tags may not be used. This is required if `source_address_prefix` is not specified.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefixes")]
    pub r#source_address_prefixes: Option<Vec<String>>,
    /// A List of source Application Security Group IDs
    #[builder(into)]
    #[serde(rename = "sourceApplicationSecurityGroupIds")]
    pub r#source_application_security_group_ids: Option<Vec<String>>,
    /// Source Port or Range. Integer or range between `0` and `65535` or `*` to match any. This is required if `source_port_ranges` is not specified.
    #[builder(into)]
    #[serde(rename = "sourcePortRange")]
    pub r#source_port_range: Option<String>,
    /// List of source ports or port ranges. This is required if `source_port_range` is not specified.
    #[builder(into)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Option<Vec<String>>,
}
