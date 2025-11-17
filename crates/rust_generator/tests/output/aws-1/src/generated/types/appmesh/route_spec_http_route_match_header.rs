#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteSpecHttpRouteMatchHeader {
    /// If `true`, the match is on the opposite of the `match` method and value. Default is `false`.
    #[builder(into)]
    #[serde(rename = "invert")]
    pub r#invert: Option<bool>,
    /// Method and value to match the header value sent with a request. Specify one match method.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Option<Box<super::super::types::appmesh::RouteSpecHttpRouteMatchHeaderMatch>>,
    /// Name for the HTTP header in the client request that will be matched on.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
