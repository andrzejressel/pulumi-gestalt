#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyFirewallPolicyPolicyVariablesRuleVariableIpSet {
    /// Set of IPv4 or IPv6 addresses in CIDR notation to use for the Suricata `HOME_NET` variable.
    #[builder(into)]
    #[serde(rename = "definitions")]
    pub r#definitions: Vec<String>,
}
