#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGatewayRouteSpecHttp2RouteAction {
    #[builder(into)]
    #[serde(rename = "rewrites")]
    pub r#rewrites: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteActionRewrite>>,
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteActionTarget>>,
}
