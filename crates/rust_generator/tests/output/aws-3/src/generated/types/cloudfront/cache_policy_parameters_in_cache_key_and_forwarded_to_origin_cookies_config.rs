#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig {
    /// Whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values for `cookie_behavior` are `none`, `whitelist`, `allExcept`, and `all`.
    #[builder(into)]
    #[serde(rename = "cookieBehavior")]
    pub r#cookie_behavior: String,
    /// Object that contains a list of cookie names. See Items for more information.
    #[builder(into)]
    #[serde(rename = "cookies")]
    pub r#cookies: Option<Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookies>>,
}
