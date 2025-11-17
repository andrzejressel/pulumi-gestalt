#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyCustomRuleMatchCondition {
    /// A list of match values. This is **Required** when the `operator` is not `Any`.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Option<Vec<String>>,
    /// One or more `match_variables` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "matchVariables")]
    pub r#match_variables: Vec<super::super::types::waf::PolicyCustomRuleMatchConditionMatchVariable>,
    /// Describes if this is negate condition or not
    #[builder(into)]
    #[serde(rename = "negationCondition")]
    pub r#negation_condition: Option<bool>,
    /// Describes operator to be matched. Possible values are `Any`, `IPMatch`, `GeoMatch`, `Equal`, `Contains`, `LessThan`, `GreaterThan`, `LessThanOrEqual`, `GreaterThanOrEqual`, `BeginsWith`, `EndsWith` and `Regex`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// A list of transformations to do before the match is attempted. Possible values are `HtmlEntityDecode`, `Lowercase`, `RemoveNulls`, `Trim`, `Uppercase`, `UrlDecode` and `UrlEncode`.
    #[builder(into)]
    #[serde(rename = "transforms")]
    pub r#transforms: Option<Vec<String>>,
}
