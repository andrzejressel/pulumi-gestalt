#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGatewayRouteSpecHttp2RouteMatchHeaderMatch {
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: String,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: String,
    #[builder(into)]
    #[serde(rename = "ranges")]
    pub r#ranges: Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchHeaderMatchRange>,
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: String,
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: String,
}
