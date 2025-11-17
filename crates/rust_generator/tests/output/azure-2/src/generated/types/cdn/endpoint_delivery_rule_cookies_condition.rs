#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeliveryRuleCookiesCondition {
    /// List of values for the cookie. This is required if `operator` is not `Any`.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Option<Vec<String>>,
    /// Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Option<bool>,
    /// Valid values are `Any`, `BeginsWith`, `Contains`, `EndsWith`, `Equal`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan` and `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Name of the cookie.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: String,
    /// A list of transforms. Valid values are `Lowercase` and `Uppercase`.
    #[builder(into)]
    #[serde(rename = "transforms")]
    pub r#transforms: Option<Vec<String>>,
}
