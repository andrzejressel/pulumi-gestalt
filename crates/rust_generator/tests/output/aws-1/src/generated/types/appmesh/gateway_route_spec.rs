#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GatewayRouteSpec {
    /// Specification of a gRPC gateway route.
    #[builder(into)]
    #[serde(rename = "grpcRoute")]
    pub r#grpc_route: Box<Option<super::super::types::appmesh::GatewayRouteSpecGrpcRoute>>,
    /// Specification of an HTTP/2 gateway route.
    #[builder(into)]
    #[serde(rename = "http2Route")]
    pub r#http_2_route: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttp2Route>>,
    /// Specification of an HTTP gateway route.
    #[builder(into)]
    #[serde(rename = "httpRoute")]
    pub r#http_route: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRoute>>,
    /// Priority for the gateway route, between `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
}
