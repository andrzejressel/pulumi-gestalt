#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig {
    /// Determines whether any URL query strings in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are `none`, `whitelist`, `allExcept`, `all`.
    #[builder(into)]
    #[serde(rename = "queryStringBehavior")]
    pub r#query_string_behavior: String,
    /// Object that contains a list of query string names. See Items for more information.
    #[builder(into)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryString>,
}
