#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyRuleCollectionGroupNatRuleCollectionRule {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The destination IP address (including CIDR).
    #[builder(into)]
    #[serde(rename = "destinationAddress")]
    pub r#destination_address: Option<String>,
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Option<String>,
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
    /// Specifies the translated address.
    #[builder(into)]
    #[serde(rename = "translatedAddress")]
    pub r#translated_address: Option<String>,
    /// Specifies the translated FQDN.
    /// 
    /// > **NOTE:** Exactly one of `translated_address` and `translated_fqdn` should be set.
    #[builder(into)]
    #[serde(rename = "translatedFqdn")]
    pub r#translated_fqdn: Option<String>,
    /// Specifies the translated port.
    #[builder(into)]
    #[serde(rename = "translatedPort")]
    pub r#translated_port: i32,
}
