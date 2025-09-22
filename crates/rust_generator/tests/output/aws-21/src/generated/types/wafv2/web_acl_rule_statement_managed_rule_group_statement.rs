#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementManagedRuleGroupStatement {
    /// Additional information that's used by a managed rule group. Only one rule attribute is allowed in each config. See `managed_rule_group_configs` for more details
    #[builder(into)]
    #[serde(rename = "managedRuleGroupConfigs")]
    pub r#managed_rule_group_configs: Option<Vec<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfig>>,
    /// Name of the managed rule group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Action settings to use in the place of the rule actions that are configured inside the rule group. You specify one override for each rule whose action you want to change. See `rule_action_override` below for details.
    #[builder(into)]
    #[serde(rename = "ruleActionOverrides")]
    pub r#rule_action_overrides: Option<Vec<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementRuleActionOverride>>,
    /// Narrows the scope of the statement to matching web requests. This can be any nestable statement, and you can nest statements at any level below this scope-down statement. See `statement` above for details.
    #[builder(into)]
    #[serde(rename = "scopeDownStatement")]
    pub r#scope_down_statement: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatement>>,
    /// Name of the managed rule group vendor.
    #[builder(into)]
    #[serde(rename = "vendorName")]
    pub r#vendor_name: String,
    /// Version of the managed rule group. You can set `Version_1.0` or `Version_1.1` etc. If you want to use the default version, do not set anything.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
