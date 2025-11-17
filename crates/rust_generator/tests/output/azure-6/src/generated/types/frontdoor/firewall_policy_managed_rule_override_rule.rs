#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyManagedRuleOverrideRule {
    /// The action to be applied when the rule matches. Possible values are `Allow`, `Block`, `Log`, or `Redirect`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Is the managed rule override enabled or disabled. Defaults to `false`
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<super::super::types::frontdoor::FirewallPolicyManagedRuleOverrideRuleExclusion>>,
    /// Identifier for the managed rule.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: String,
}
