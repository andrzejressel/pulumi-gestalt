#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RouteSpecHttpRoute {
    /// Action to take if a match is determined.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::appmesh::RouteSpecHttpRouteAction>,
    /// Criteria for determining an HTTP request match.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::appmesh::RouteSpecHttpRouteMatch>,
    /// Retry policy.
    #[builder(into, default)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Box<Option<super::super::types::appmesh::RouteSpecHttpRouteRetryPolicy>>,
    /// Types of timeouts.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<super::super::types::appmesh::RouteSpecHttpRouteTimeout>>,
}
