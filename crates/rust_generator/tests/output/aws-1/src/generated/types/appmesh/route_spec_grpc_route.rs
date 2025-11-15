#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteSpecGrpcRoute {
    /// Action to take if a match is determined.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::appmesh::RouteSpecGrpcRouteAction>,
    /// Criteria for determining an gRPC request match.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Option<Box<super::super::types::appmesh::RouteSpecGrpcRouteMatch>>,
    /// Retry policy.
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Option<Box<super::super::types::appmesh::RouteSpecGrpcRouteRetryPolicy>>,
    /// Types of timeouts.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<Box<super::super::types::appmesh::RouteSpecGrpcRouteTimeout>>,
}
