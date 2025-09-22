#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinitionMatchAttributes {
    /// Set of configuration blocks describing the destination ports to inspect for. If not specified, this matches with any destination port. See Destination Port below for details.
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinitionMatchAttributesDestinationPort>>,
    /// Set of configuration blocks describing the destination IP address and address ranges to inspect for, in CIDR notation. If not specified, this matches with any destination address. See Destination below for details.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinitionMatchAttributesDestination>>,
    /// Set of protocols to inspect for, specified using the protocol's assigned internet protocol number (IANA). If not specified, this matches with any protocol.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Option<Vec<i32>>,
    /// Set of configuration blocks describing the source ports to inspect for. If not specified, this matches with any source port. See Source Port below for details.
    #[builder(into)]
    #[serde(rename = "sourcePorts")]
    pub r#source_ports: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinitionMatchAttributesSourcePort>>,
    /// Set of configuration blocks describing the source IP address and address ranges to inspect for, in CIDR notation. If not specified, this matches with any source address. See Source below for details.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinitionMatchAttributesSource>>,
    /// Set of configuration blocks containing the TCP flags and masks to inspect for. If not specified, this matches with any settings.
    #[builder(into)]
    #[serde(rename = "tcpFlags")]
    pub r#tcp_flags: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRuleRuleDefinitionMatchAttributesTcpFlag>>,
}
