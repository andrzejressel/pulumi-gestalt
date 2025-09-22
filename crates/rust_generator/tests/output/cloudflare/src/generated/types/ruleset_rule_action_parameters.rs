#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleActionParameters {
    /// Specifies uncommon ports to allow cacheable assets to be served from.
    #[builder(into)]
    #[serde(rename = "additionalCacheablePorts")]
    pub r#additional_cacheable_ports: Option<Vec<i32>>,
    /// Compression algorithms to use in order of preference.
    #[builder(into)]
    #[serde(rename = "algorithms")]
    pub r#algorithms: Option<Vec<super::types::RulesetRuleActionParametersAlgorithm>>,
    /// Turn on or off Cloudflare Automatic HTTPS rewrites.
    #[builder(into)]
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Option<bool>,
    /// Indicate which file extensions to minify automatically.
    #[builder(into)]
    #[serde(rename = "autominifies")]
    pub r#autominifies: Option<Vec<super::types::RulesetRuleActionParametersAutominify>>,
    /// Inspect the visitor's browser for headers commonly associated with spammers and certain bots.
    #[builder(into)]
    #[serde(rename = "bic")]
    pub r#bic: Option<bool>,
    /// List of browser TTL parameters to apply to the request.
    #[builder(into)]
    #[serde(rename = "browserTtl")]
    pub r#browser_ttl: Box<Option<super::types::RulesetRuleActionParametersBrowserTtl>>,
    /// Whether to cache if expression matches.
    #[builder(into)]
    #[serde(rename = "cache")]
    pub r#cache: Option<bool>,
    /// List of cache key parameters to apply to the request.
    #[builder(into)]
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Box<Option<super::types::RulesetRuleActionParametersCacheKey>>,
    /// List of cache reserve parameters to apply to the request.
    #[builder(into)]
    #[serde(rename = "cacheReserve")]
    pub r#cache_reserve: Box<Option<super::types::RulesetRuleActionParametersCacheReserve>>,
    /// Content of the custom error response.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// Content-Type of the custom error response.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    /// List of cookie values to include as part of custom fields logging.
    #[builder(into)]
    #[serde(rename = "cookieFields")]
    pub r#cookie_fields: Option<Vec<String>>,
    /// Turn off all active Cloudflare Apps.
    #[builder(into)]
    #[serde(rename = "disableApps")]
    pub r#disable_apps: Option<bool>,
    /// Turn off railgun feature of the Cloudflare Speed app.
    #[builder(into)]
    #[serde(rename = "disableRailgun")]
    pub r#disable_railgun: Option<bool>,
    /// Turn off RUM feature.
    #[builder(into)]
    #[serde(rename = "disableRum")]
    pub r#disable_rum: Option<bool>,
    /// Turn off zaraz feature.
    #[builder(into)]
    #[serde(rename = "disableZaraz")]
    pub r#disable_zaraz: Option<bool>,
    /// List of edge TTL parameters to apply to the request.
    #[builder(into)]
    #[serde(rename = "edgeTtl")]
    pub r#edge_ttl: Box<Option<super::types::RulesetRuleActionParametersEdgeTtl>>,
    /// Turn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app.
    #[builder(into)]
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Option<bool>,
    /// Toggle fonts.
    #[builder(into)]
    #[serde(rename = "fonts")]
    pub r#fonts: Option<bool>,
    /// Use a list to lookup information for the action.
    #[builder(into)]
    #[serde(rename = "fromList")]
    pub r#from_list: Box<Option<super::types::RulesetRuleActionParametersFromList>>,
    /// Use a value to lookup information for the action.
    #[builder(into)]
    #[serde(rename = "fromValue")]
    pub r#from_value: Box<Option<super::types::RulesetRuleActionParametersFromValue>>,
    /// List of HTTP header modifications to perform in the ruleset rule. Note: Headers are order dependent and must be provided sorted alphabetically ascending based on the `name` value.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::types::RulesetRuleActionParametersHeader>>,
    /// Host Header that request origin receives.
    #[builder(into)]
    #[serde(rename = "hostHeader")]
    pub r#host_header: Option<String>,
    /// Turn on or off the hotlink protection feature.
    #[builder(into)]
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Option<bool>,
    /// Identifier of the action parameter to modify.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[builder(into)]
    #[serde(rename = "increment")]
    pub r#increment: Option<i32>,
    /// List of properties to configure WAF payload logging.
    #[builder(into)]
    #[serde(rename = "matchedData")]
    pub r#matched_data: Box<Option<super::types::RulesetRuleActionParametersMatchedData>>,
    /// Turn on or off Cloudflare Mirage of the Cloudflare Speed app.
    #[builder(into)]
    #[serde(rename = "mirage")]
    pub r#mirage: Option<bool>,
    /// Turn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[builder(into)]
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Option<bool>,
    /// List of properties to change request origin.
    #[builder(into)]
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<super::types::RulesetRuleActionParametersOrigin>>,
    /// Enable or disable the use of a more compliant Cache Control parsing mechanism, enabled by default for most zones.
    #[builder(into)]
    #[serde(rename = "originCacheControl")]
    pub r#origin_cache_control: Option<bool>,
    /// Pass-through error page for origin.
    #[builder(into)]
    #[serde(rename = "originErrorPagePassthru")]
    pub r#origin_error_page_passthru: Option<bool>,
    /// List of override configurations to apply to the ruleset.
    #[builder(into)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<super::types::RulesetRuleActionParametersOverrides>>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    #[builder(into)]
    #[serde(rename = "phases")]
    pub r#phases: Option<Vec<String>>,
    /// Apply options from the Polish feature of the Cloudflare Speed app.
    #[builder(into)]
    #[serde(rename = "polish")]
    pub r#polish: Option<String>,
    /// Products to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`.
    #[builder(into)]
    #[serde(rename = "products")]
    pub r#products: Option<Vec<String>>,
    /// Specifies a maximum timeout for reading content from an origin server.
    #[builder(into)]
    #[serde(rename = "readTimeout")]
    pub r#read_timeout: Option<i32>,
    /// List of request headers to include as part of custom fields logging, in lowercase.
    #[builder(into)]
    #[serde(rename = "requestFields")]
    pub r#request_fields: Option<Vec<String>>,
    /// Respect strong ETags.
    #[builder(into)]
    #[serde(rename = "respectStrongEtags")]
    pub r#respect_strong_etags: Option<bool>,
    /// List of response headers to include as part of custom fields logging, in lowercase.
    #[builder(into)]
    #[serde(rename = "responseFields")]
    pub r#response_fields: Option<Vec<String>>,
    /// List of parameters that configure the response given to end users.
    #[builder(into)]
    #[serde(rename = "responses")]
    pub r#responses: Option<Vec<super::types::RulesetRuleActionParametersResponse>>,
    /// Turn on or off Cloudflare Rocket Loader in the Cloudflare Speed app.
    #[builder(into)]
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Option<bool>,
    /// Map of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { "efb7b8c949ac4650a09736fc376e9aee" = "5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760" }`.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Option<std::collections::HashMap<String, String>>,
    /// Which ruleset ID to target.
    #[builder(into)]
    #[serde(rename = "ruleset")]
    pub r#ruleset: Option<String>,
    /// List of managed WAF rule IDs to target. Only valid when the `"action"` is set to skip.
    #[builder(into)]
    #[serde(rename = "rulesets")]
    pub r#rulesets: Option<Vec<String>>,
    /// Control options for the Security Level feature from the Security app.
    #[builder(into)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    /// List of serve stale parameters to apply to the request.
    #[builder(into)]
    #[serde(rename = "serveStale")]
    pub r#serve_stale: Box<Option<super::types::RulesetRuleActionParametersServeStale>>,
    /// Turn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app.
    #[builder(into)]
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Option<bool>,
    /// List of properties to manange Server Name Indication.
    #[builder(into)]
    #[serde(rename = "sni")]
    pub r#sni: Box<Option<super::types::RulesetRuleActionParametersSni>>,
    /// Control options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[builder(into)]
    #[serde(rename = "ssl")]
    pub r#ssl: Option<String>,
    /// HTTP status code of the custom error response.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    /// Turn on or off the SXG feature.
    #[builder(into)]
    #[serde(rename = "sxg")]
    pub r#sxg: Option<bool>,
    /// List of URI properties to configure for the ruleset rule when performing URL rewrite transformations.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<super::types::RulesetRuleActionParametersUri>>,
    /// Version of the ruleset to deploy.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
