#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RouteSpecTcpRoute {
    /// Action to take if a match is determined.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::appmesh::RouteSpecTcpRouteAction>,
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<Option<super::super::types::appmesh::RouteSpecTcpRouteMatch>>,
    /// Types of timeouts.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<super::super::types::appmesh::RouteSpecTcpRouteTimeout>>,
}
