#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionResponseHeaderToRemove {
    /// Headers to remove from the response prior to sending it back to the client.
    /// Response headers are only sent to the client, and do not have an effect on the cache serving the response.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
}
