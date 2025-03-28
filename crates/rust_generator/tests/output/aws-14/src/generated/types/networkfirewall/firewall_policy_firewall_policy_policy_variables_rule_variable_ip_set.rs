#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyFirewallPolicyPolicyVariablesRuleVariableIpSet {
    /// Set of IPv4 or IPv6 addresses in CIDR notation to use for the Suricata `HOME_NET` variable.
    #[builder(into)]
    #[serde(rename = "definitions")]
    pub r#definitions: Box<Vec<String>>,
}
