#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleGroupRulesSourceRulesSourceList {
    /// String value to specify whether domains in the target list are allowed or denied access. Valid values: `ALLOWLIST`, `DENYLIST`.
    #[builder(into)]
    #[serde(rename = "generatedRulesType")]
    pub r#generated_rules_type: String,
    /// Set of types of domain specifications that are provided in the `targets` argument. Valid values: `HTTP_HOST`, `TLS_SNI`.
    #[builder(into)]
    #[serde(rename = "targetTypes")]
    pub r#target_types: Vec<String>,
    /// Set of domains that you want to inspect for in your traffic flows.
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Vec<String>,
}
