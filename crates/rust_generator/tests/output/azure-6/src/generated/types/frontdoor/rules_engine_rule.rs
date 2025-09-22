#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesEngineRule {
    /// An `action` block as defined below.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::frontdoor::RulesEngineRuleAction>>,
    /// One or more `match_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "matchConditions")]
    pub r#match_conditions: Option<Vec<super::super::types::frontdoor::RulesEngineRuleMatchCondition>>,
    /// The name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Priority of the rule, must be unique per rules engine definition.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
}
