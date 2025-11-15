#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteMapRuleMatchCriterion {
    /// A list of AS paths which this criterion matches.
    #[builder(into)]
    #[serde(rename = "asPaths")]
    pub r#as_paths: Option<Vec<String>>,
    /// A list of BGP communities which this criterion matches.
    #[builder(into)]
    #[serde(rename = "communities")]
    pub r#communities: Option<Vec<String>>,
    /// The match condition to apply the rule of the Route Map. Possible values are `Contains`, `Equals`, `NotContains`, `NotEquals` and `Unknown`.
    #[builder(into)]
    #[serde(rename = "matchCondition")]
    pub r#match_condition: String,
    /// A list of route prefixes which this criterion matches.
    #[builder(into)]
    #[serde(rename = "routePrefixes")]
    pub r#route_prefixes: Option<Vec<String>>,
}
