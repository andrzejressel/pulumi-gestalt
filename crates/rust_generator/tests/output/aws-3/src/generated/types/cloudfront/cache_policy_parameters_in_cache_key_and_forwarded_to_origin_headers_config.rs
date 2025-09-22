#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig {
    /// Whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values for `header_behavior` are `none` and `whitelist`.
    #[builder(into)]
    #[serde(rename = "headerBehavior")]
    pub r#header_behavior: Option<String>,
    /// Object contains a list of header names. See Items for more information.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaders>>,
}
