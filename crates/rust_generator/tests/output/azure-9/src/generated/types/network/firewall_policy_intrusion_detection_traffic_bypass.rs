#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyIntrusionDetectionTrafficBypass {
    /// The description for this bypass traffic setting.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Specifies a list of destination IP addresses that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Option<Vec<String>>,
    /// Specifies a list of destination IP groups that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "destinationIpGroups")]
    pub r#destination_ip_groups: Option<Vec<String>>,
    /// Specifies a list of destination IP ports that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Option<Vec<String>>,
    /// The name which should be used for this bypass traffic setting.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The protocols any of `ANY`, `TCP`, `ICMP`, `UDP` that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Specifies a list of source addresses that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    /// Specifies a list of source IP groups that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Option<Vec<String>>,
}
