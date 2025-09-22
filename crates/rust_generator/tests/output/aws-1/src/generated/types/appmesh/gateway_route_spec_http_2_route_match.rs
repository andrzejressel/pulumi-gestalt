#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GatewayRouteSpecHttp2RouteMatch {
    /// Client request headers to match on.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchHeader>>,
    /// Host name to match on.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchHostname>>,
    /// Client request path to match on.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchPath>>,
    /// The port number to match from the request.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Path to match requests with. This parameter must always start with `/`, which by itself matches all requests to the virtual service name.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Client request query parameters to match on.
    #[builder(into)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Option<Vec<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchQueryParameter>>,
}
