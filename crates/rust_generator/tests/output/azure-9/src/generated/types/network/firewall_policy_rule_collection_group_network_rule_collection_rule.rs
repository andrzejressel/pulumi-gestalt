#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyRuleCollectionGroupNetworkRuleCollectionRule {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "destinationFqdns")]
    pub r#destination_fqdns: Option<Vec<String>>,
    /// Specifies a list of destination IP groups.
    #[builder(into)]
    #[serde(rename = "destinationIpGroups")]
    pub r#destination_ip_groups: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Vec<String>,
    /// The name which should be used for this Firewall Policy Rule Collection Group. Changing this forces a new Firewall Policy Rule Collection Group to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Vec<String>,
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Option<Vec<String>>,
}
