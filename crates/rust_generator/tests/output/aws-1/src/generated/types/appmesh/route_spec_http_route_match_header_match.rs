#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RouteSpecHttpRouteMatchHeaderMatch {
    /// Header value sent by the client must match the specified value exactly.
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: Option<String>,
    /// Header value sent by the client must begin with the specified characters.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Object that specifies the range of numbers that the header value sent by the client must be included in.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Box<Option<super::super::types::appmesh::RouteSpecHttpRouteMatchHeaderMatchRange>>,
    /// Header value sent by the client must include the specified characters.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<String>,
    /// Header value sent by the client must end with the specified characters.
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Option<String>,
}
