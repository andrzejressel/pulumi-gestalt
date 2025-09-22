#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyManagedRulesExclusion {
    /// One or more `excluded_rule_set` block defined below.
    #[builder(into)]
    #[serde(rename = "excludedRuleSet")]
    pub r#excluded_rule_set: Option<Box<super::super::types::waf::PolicyManagedRulesExclusionExcludedRuleSet>>,
    /// The name of the Match Variable. Possible values: `RequestArgKeys`, `RequestArgNames`, `RequestArgValues`, `RequestCookieKeys`, `RequestCookieNames`, `RequestCookieValues`, `RequestHeaderKeys`, `RequestHeaderNames`, `RequestHeaderValues`.
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: String,
    /// Describes field of the matchVariable collection.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: String,
    /// Describes operator to be matched. Possible values: `Contains`, `EndsWith`, `Equals`, `EqualsAny`, `StartsWith`.
    #[builder(into)]
    #[serde(rename = "selectorMatchOperator")]
    pub r#selector_match_operator: String,
}
