#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteSpec {
    /// GRPC routing information for the route.
    #[builder(into)]
    #[serde(rename = "grpcRoute")]
    pub r#grpc_route: Option<Box<super::super::types::appmesh::RouteSpecGrpcRoute>>,
    /// HTTP/2 routing information for the route.
    #[builder(into)]
    #[serde(rename = "http2Route")]
    pub r#http_2_route: Option<Box<super::super::types::appmesh::RouteSpecHttp2Route>>,
    /// HTTP routing information for the route.
    #[builder(into)]
    #[serde(rename = "httpRoute")]
    pub r#http_route: Option<Box<super::super::types::appmesh::RouteSpecHttpRoute>>,
    /// Priority for the route, between `0` and `1000`.
    /// Routes are matched based on the specified value, where `0` is the highest priority.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    /// TCP routing information for the route.
    #[builder(into)]
    #[serde(rename = "tcpRoute")]
    pub r#tcp_route: Option<Box<super::super::types::appmesh::RouteSpecTcpRoute>>,
}
