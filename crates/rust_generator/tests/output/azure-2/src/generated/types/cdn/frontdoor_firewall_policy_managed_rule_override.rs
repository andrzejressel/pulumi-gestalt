#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorFirewallPolicyManagedRuleOverride {
    /// One or more `exclusion` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRuleOverrideExclusion>>,
    /// The managed rule group to override.
    #[builder(into)]
    #[serde(rename = "ruleGroupName")]
    pub r#rule_group_name: String,
    /// One or more `rule` blocks as defined below. If none are specified, all of the rules in the group will be disabled.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRuleOverrideRule>>,
}
