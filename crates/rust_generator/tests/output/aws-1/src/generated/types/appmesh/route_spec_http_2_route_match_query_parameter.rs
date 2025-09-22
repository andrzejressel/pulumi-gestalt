#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RouteSpecHttp2RouteMatchQueryParameter {
    /// The query parameter to match on.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Option<Box<super::super::types::appmesh::RouteSpecHttp2RouteMatchQueryParameterMatch>>,
    /// Name for the query parameter that will be matched on.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
