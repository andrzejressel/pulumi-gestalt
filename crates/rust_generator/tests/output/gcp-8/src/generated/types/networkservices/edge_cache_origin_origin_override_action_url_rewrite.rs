#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheOriginOriginOverrideActionUrlRewrite {
    /// Prior to forwarding the request to the selected
    /// origin, the request's host header is replaced with
    /// contents of the hostRewrite.
    /// This value must be between 1 and 255 characters.
    #[builder(into)]
    #[serde(rename = "hostRewrite")]
    pub r#host_rewrite: Option<String>,
}
