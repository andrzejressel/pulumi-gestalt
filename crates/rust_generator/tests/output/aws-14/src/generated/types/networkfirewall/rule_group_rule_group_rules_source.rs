#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleGroupRulesSource {
    /// A configuration block containing **stateful** inspection criteria for a domain list rule group. See Rules Source List below for details.
    #[builder(into)]
    #[serde(rename = "rulesSourceList")]
    pub r#rules_source_list: Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceRulesSourceList>>,
    /// The fully qualified name of a file in an S3 bucket that contains Suricata compatible intrusion preventions system (IPS) rules or the Suricata rules as a string. These rules contain **stateful** inspection criteria and the action to take for traffic that matches the criteria.
    #[builder(into)]
    #[serde(rename = "rulesString")]
    pub r#rules_string: Option<String>,
    /// Set of configuration blocks containing **stateful** inspection criteria for 5-tuple rules to be used together in a rule group. See Stateful Rule below for details.
    #[builder(into)]
    #[serde(rename = "statefulRules")]
    pub r#stateful_rules: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRule>>,
    /// A configuration block containing **stateless** inspection criteria for a stateless rule group. See Stateless Rules and Custom Actions below for details.
    #[builder(into)]
    #[serde(rename = "statelessRulesAndCustomActions")]
    pub r#stateless_rules_and_custom_actions: Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActions>>,
}
