#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFirewallPolicyFirewallPolicy {
    #[builder(into)]
    #[serde(rename = "statefulDefaultActions")]
    pub r#stateful_default_actions: Vec<String>,
    #[builder(into)]
    #[serde(rename = "statefulEngineOptions")]
    pub r#stateful_engine_options: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatefulEngineOption>,
    #[builder(into)]
    #[serde(rename = "statefulRuleGroupReferences")]
    pub r#stateful_rule_group_references: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatefulRuleGroupReference>,
    #[builder(into)]
    #[serde(rename = "statelessCustomActions")]
    pub r#stateless_custom_actions: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessCustomAction>,
    #[builder(into)]
    #[serde(rename = "statelessDefaultActions")]
    pub r#stateless_default_actions: Vec<String>,
    #[builder(into)]
    #[serde(rename = "statelessFragmentDefaultActions")]
    pub r#stateless_fragment_default_actions: Vec<String>,
    #[builder(into)]
    #[serde(rename = "statelessRuleGroupReferences")]
    pub r#stateless_rule_group_references: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessRuleGroupReference>,
    #[builder(into)]
    #[serde(rename = "tlsInspectionConfigurationArn")]
    pub r#tls_inspection_configuration_arn: String,
}
