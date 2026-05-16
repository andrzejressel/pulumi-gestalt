#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
