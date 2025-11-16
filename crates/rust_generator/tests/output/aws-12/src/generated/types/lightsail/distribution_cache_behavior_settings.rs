#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionCacheBehaviorSettings {
    /// The HTTP methods that are processed and forwarded to the distribution's origin.
    #[builder(into)]
    #[serde(rename = "allowedHttpMethods")]
    pub r#allowed_http_methods: Option<String>,
    /// The HTTP method responses that are cached by your distribution.
    #[builder(into)]
    #[serde(rename = "cachedHttpMethods")]
    pub r#cached_http_methods: Option<String>,
    /// The default amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the content has been updated.
    #[builder(into)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Option<i32>,
    /// An object that describes the cookies that are forwarded to the origin. Your content is cached based on the cookies that are forwarded. Detailed below
    #[builder(into)]
    #[serde(rename = "forwardedCookies")]
    pub r#forwarded_cookies: Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedCookies>>,
    /// An object that describes the headers that are forwarded to the origin. Your content is cached based on the headers that are forwarded. Detailed below
    #[builder(into)]
    #[serde(rename = "forwardedHeaders")]
    pub r#forwarded_headers: Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedHeaders>>,
    /// An object that describes the query strings that are forwarded to the origin. Your content is cached based on the query strings that are forwarded. Detailed below
    #[builder(into)]
    #[serde(rename = "forwardedQueryStrings")]
    pub r#forwarded_query_strings: Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedQueryStrings>>,
    /// The maximum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated.
    #[builder(into)]
    #[serde(rename = "maximumTtl")]
    pub r#maximum_ttl: Option<i32>,
    /// The minimum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated.
    #[builder(into)]
    #[serde(rename = "minimumTtl")]
    pub r#minimum_ttl: Option<i32>,
}
