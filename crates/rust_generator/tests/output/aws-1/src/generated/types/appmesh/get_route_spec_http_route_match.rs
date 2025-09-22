#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRouteSpecHttpRouteMatch {
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatchHeader>,
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: String,
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatchPath>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: String,
    #[builder(into)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatchQueryParameter>,
    #[builder(into)]
    #[serde(rename = "scheme")]
    pub r#scheme: String,
}
