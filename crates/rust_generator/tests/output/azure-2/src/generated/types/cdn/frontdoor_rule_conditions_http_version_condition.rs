#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorRuleConditionsHttpVersionCondition {
    /// What HTTP version should this condition match? Possible values `2.0`, `1.1`, `1.0` or `0.9`.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Vec<String>,
    /// If `true` operator becomes the opposite of its value. Possible values `true` or `false`. Defaults to `false`. Details can be found in the `Condition Operator List` below.
    #[builder(into)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Option<bool>,
    /// Possible value `Equal`. Defaults to `Equal`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
}
