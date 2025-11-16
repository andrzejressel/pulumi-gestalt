#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorFirewallPolicyCustomRuleMatchCondition {
    /// Up to `600` possible values to match. Limit is in total across all `match_condition` blocks and `match_values` arguments. String value itself can be up to `256` characters in length.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Vec<String>,
    /// The request variable to compare with. Possible values are `Cookies`, `PostArgs`, `QueryString`, `RemoteAddr`, `RequestBody`, `RequestHeader`, `RequestMethod`, `RequestUri`, or `SocketAddr`.
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: String,
    /// Should the result of the condition be negated.
    #[builder(into)]
    #[serde(rename = "negationCondition")]
    pub r#negation_condition: Option<bool>,
    /// Comparison type to use for matching with the variable value. Possible values are `Any`, `BeginsWith`, `Contains`, `EndsWith`, `Equal`, `GeoMatch`, `GreaterThan`, `GreaterThanOrEqual`, `IPMatch`, `LessThan`, `LessThanOrEqual` or `RegEx`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Match against a specific key if the `match_variable` is `QueryString`, `PostArgs`, `RequestHeader` or `Cookies`.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Option<String>,
    /// Up to `5` transforms to apply. Possible values are `Lowercase`, `RemoveNulls`, `Trim`, `Uppercase`, `URLDecode` or `URLEncode`.
    #[builder(into)]
    #[serde(rename = "transforms")]
    pub r#transforms: Option<Vec<String>>,
}
