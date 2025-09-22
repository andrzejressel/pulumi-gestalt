#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActions {
    /// Set of configuration blocks containing custom action definitions that are available for use by the set of `stateless rule`. See Custom Action below for details.
    #[builder(into)]
    #[serde(rename = "customActions")]
    pub r#custom_actions: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomAction>>,
    /// Set of configuration blocks containing the stateless rules for use in the stateless rule group. See Stateless Rule below for details.
    #[builder(into)]
    #[serde(rename = "statelessRules")]
    pub r#stateless_rules: Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRule>,
}
