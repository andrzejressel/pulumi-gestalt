#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RouteMapRuleActionParameter {
    /// A list of AS paths.
    #[builder(into)]
    #[serde(rename = "asPaths")]
    pub r#as_paths: Option<Vec<String>>,
    /// A list of BGP communities.
    #[builder(into)]
    #[serde(rename = "communities")]
    pub r#communities: Option<Vec<String>>,
    /// A list of route prefixes.
    #[builder(into)]
    #[serde(rename = "routePrefixes")]
    pub r#route_prefixes: Option<Vec<String>>,
}
