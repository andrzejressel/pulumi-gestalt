#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionDefaultCacheBehaviorForwardedValues {
    /// The forwarded values cookies that specifies how CloudFront handles cookies (maximum one).
    #[builder(into)]
    #[serde(rename = "cookies")]
    pub r#cookies: Box<super::super::types::cloudfront::DistributionDefaultCacheBehaviorForwardedValuesCookies>,
    /// Headers, if any, that you want CloudFront to vary upon for this cache behavior. Specify `*` to include all headers.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<String>>,
    /// Indicates whether you want CloudFront to forward query strings to the origin that is associated with this cache behavior.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: bool,
    /// When specified, along with a value of `true` for `query_string`, all query strings are forwarded, however only the query string keys listed in this argument are cached. When omitted with a value of `true` for `query_string`, all query string keys are cached.
    #[builder(into)]
    #[serde(rename = "queryStringCacheKeys")]
    pub r#query_string_cache_keys: Option<Vec<String>>,
}
