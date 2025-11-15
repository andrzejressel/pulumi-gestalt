#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteSpecGrpcRouteMatch {
    /// Data to match from the gRPC request.
    #[builder(into)]
    #[serde(rename = "metadatas")]
    pub r#metadatas: Option<Vec<super::super::types::appmesh::RouteSpecGrpcRouteMatchMetadata>>,
    /// Method name to match from the request. If you specify a name, you must also specify a `service_name`.
    #[builder(into)]
    #[serde(rename = "methodName")]
    pub r#method_name: Option<String>,
    /// The port number to match from the request.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Fully qualified domain name for the service to match from the request.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Option<String>,
}
