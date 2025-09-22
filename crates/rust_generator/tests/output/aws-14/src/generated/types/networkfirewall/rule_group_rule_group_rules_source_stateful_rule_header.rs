#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleGroupRulesSourceStatefulRuleHeader {
    /// The destination IP address or address range to inspect for, in CIDR notation. To match with any address, specify `ANY`.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: String,
    /// The destination port to inspect for. To match with any address, specify `ANY`.
    #[builder(into)]
    #[serde(rename = "destinationPort")]
    pub r#destination_port: String,
    /// The direction of traffic flow to inspect. Valid values: `ANY` or `FORWARD`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: String,
    /// The protocol to inspect. Valid values: `IP`, `TCP`, `UDP`, `ICMP`, `HTTP`, `FTP`, `TLS`, `SMB`, `DNS`, `DCERPC`, `SSH`, `SMTP`, `IMAP`, `MSN`, `KRB5`, `IKEV2`, `TFTP`, `NTP`, `DHCP`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The source IP address or address range for, in CIDR notation. To match with any address, specify `ANY`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
    /// The source port to inspect for. To match with any address, specify `ANY`.
    #[builder(into)]
    #[serde(rename = "sourcePort")]
    pub r#source_port: String,
}
