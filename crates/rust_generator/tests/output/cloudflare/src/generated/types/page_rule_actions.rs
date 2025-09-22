#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PageRuleActions {
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into)]
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Option<bool>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Option<String>,
    /// The Time To Live for the browser cache. `0` means 'Respect Existing Headers'
    #[builder(into)]
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Option<String>,
    /// String value of cookie name to conditionally bypass cache the page.
    #[builder(into)]
    #[serde(rename = "bypassCacheOnCookie")]
    pub r#bypass_cache_on_cookie: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Option<String>,
    /// Controls how Cloudflare creates Cache Keys used to identify files in cache. See below for full description.
    #[builder(into)]
    #[serde(rename = "cacheKeyFields")]
    pub r#cache_key_fields: Option<Box<super::types::PageRuleActionsCacheKeyFields>>,
    /// Whether to set the cache level to `"bypass"`, `"basic"`, `"simplified"`, `"aggressive"`, or `"cache_everything"`.
    #[builder(into)]
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Option<String>,
    /// String value of cookie name to conditionally cache the page.
    #[builder(into)]
    #[serde(rename = "cacheOnCookie")]
    pub r#cache_on_cookie: Option<String>,
    /// Set cache TTL based on the response status from the origin web server. Can be specified multiple times. See below for full description.
    #[builder(into)]
    #[serde(rename = "cacheTtlByStatuses")]
    pub r#cache_ttl_by_statuses: Option<Vec<super::types::PageRuleActionsCacheTtlByStatus>>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into)]
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Option<bool>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into)]
    #[serde(rename = "disablePerformance")]
    pub r#disable_performance: Option<bool>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into)]
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Option<bool>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into)]
    #[serde(rename = "disableSecurity")]
    pub r#disable_security: Option<bool>,
    /// Boolean of whether this action is enabled. Default: false.
    #[builder(into)]
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Option<bool>,
    /// The Time To Live for the edge cache.
    #[builder(into)]
    #[serde(rename = "edgeCacheTtl")]
    pub r#edge_cache_ttl: Option<i32>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Option<String>,
    /// Whether origin Cache-Control action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "explicitCacheControl")]
    pub r#explicit_cache_control: Option<String>,
    /// The URL to forward to, and with what status. See below.
    #[builder(into)]
    #[serde(rename = "forwardingUrl")]
    pub r#forwarding_url: Option<Box<super::types::PageRuleActionsForwardingUrl>>,
    /// Value of the Host header to send.
    #[builder(into)]
    #[serde(rename = "hostHeaderOverride")]
    pub r#host_header_override: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Option<String>,
    /// The configuration for HTML, CSS and JS minification. See below for full list of options.
    #[builder(into)]
    #[serde(rename = "minifies")]
    pub r#minifies: Option<Vec<super::types::PageRuleActionsMinify>>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "mirage")]
    pub r#mirage: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Option<String>,
    /// Whether this action is `"off"`, `"lossless"` or `"lossy"`.
    #[builder(into)]
    #[serde(rename = "polish")]
    pub r#polish: Option<String>,
    /// Overridden origin server name.
    #[builder(into)]
    #[serde(rename = "resolveOverride")]
    pub r#resolve_override: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "respectStrongEtag")]
    pub r#respect_strong_etag: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Option<String>,
    /// Whether to set the rocket loader to `"on"`, `"off"`.
    #[builder(into)]
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Option<String>,
    /// Whether to set the security level to `"off"`, `"essentially_off"`, `"low"`, `"medium"`, `"high"`, or `"under_attack"`.
    #[builder(into)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Option<String>,
    /// Whether to set the SSL mode to `"off"`, `"flexible"`, `"full"`, `"strict"`, or `"origin_pull"`.
    #[builder(into)]
    #[serde(rename = "ssl")]
    pub r#ssl: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Option<String>,
    /// Whether this action is `"on"` or `"off"`.
    #[builder(into)]
    #[serde(rename = "waf")]
    pub r#waf: Option<String>,
}
