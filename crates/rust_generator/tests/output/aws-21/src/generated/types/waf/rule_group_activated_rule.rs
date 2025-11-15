#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupActivatedRule {
    /// Specifies the action that CloudFront or AWS WAF takes when a web request matches the conditions in the rule.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::waf::RuleGroupActivatedRuleAction>,
    /// Specifies the order in which the rules are evaluated. Rules with a lower value are evaluated before rules with a higher value.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// The ID of a rule
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: String,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
