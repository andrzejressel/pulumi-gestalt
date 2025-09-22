#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyManagedRules {
    /// One or more `exclusion` block defined below.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<super::super::types::waf::PolicyManagedRulesExclusion>>,
    /// One or more `managed_rule_set` block defined below.
    #[builder(into)]
    #[serde(rename = "managedRuleSets")]
    pub r#managed_rule_sets: Vec<super::super::types::waf::PolicyManagedRulesManagedRuleSet>,
}
