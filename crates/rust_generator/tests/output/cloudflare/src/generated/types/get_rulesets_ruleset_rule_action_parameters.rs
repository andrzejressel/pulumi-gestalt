#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsRulesetRuleActionParameters {
    /// Allows for the ability to support caching on non-standard ports.
    #[builder(into)]
    pub r#additional_cacheable_ports: Option<Vec<i32>>,
    /// Turn on or off Cloudflare Automatic HTTPS rewrites.
    #[builder(into)]
    pub r#automatic_https_rewrites: Option<bool>,
    /// Indicate which file extensions to minify automatically.
    #[builder(into)]
    pub r#autominifies: Option<Vec<super::types::GetRulesetsRulesetRuleActionParametersAutominify>>,
    /// Inspect the visitor's browser for headers commonly associated with spammers and certain bots.
    #[builder(into)]
    pub r#bic: Option<bool>,
    /// List of browser TTL parameters to apply to the request.
    #[builder(into)]
    pub r#browser_ttl: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersBrowserTtl>>,
    /// Whether to cache if expression matches.
    #[builder(into)]
    pub r#cache: Option<bool>,
    /// List of cache key parameters to apply to the request.
    #[builder(into)]
    pub r#cache_key: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersCacheKey>>,
    /// List of cache reserve parameters to apply to the request.
    #[builder(into)]
    pub r#cache_reserve: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersCacheReserve>>,
    /// Content of the custom error response
    #[builder(into)]
    pub r#content: Option<String>,
    /// Content-Type of the custom error response
    #[builder(into)]
    pub r#content_type: Option<String>,
    /// List of cookie values to include as part of custom fields logging.
    #[builder(into)]
    pub r#cookie_fields: Option<Vec<String>>,
    /// Turn off all active Cloudflare Apps.
    #[builder(into)]
    pub r#disable_apps: Option<bool>,
    /// Turn off railgun feature of the Cloudflare Speed app.
    #[builder(into)]
    pub r#disable_railgun: Option<bool>,
    /// Turn off zaraz feature.
    #[builder(into)]
    pub r#disable_zaraz: Option<bool>,
    /// List of edge TTL parameters to apply to the request.
    #[builder(into)]
    pub r#edge_ttl: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersEdgeTtl>>,
    /// Turn on or off the Cloudflare Email Obfuscation feature of the Cloudflare Scrape Shield app.
    #[builder(into)]
    pub r#email_obfuscation: Option<bool>,
    /// Use a list to lookup information for the action.
    #[builder(into)]
    pub r#from_list: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersFromList>>,
    /// Use a value to lookup information for the action.
    #[builder(into)]
    pub r#from_value: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersFromValue>>,
    /// List of HTTP header modifications to perform in the ruleset rule.
    #[builder(into)]
    pub r#headers: Option<Vec<super::types::GetRulesetsRulesetRuleActionParametersHeader>>,
    /// Host Header that request origin receives.
    #[builder(into)]
    pub r#host_header: Option<String>,
    /// Turn on or off the hotlink protection feature.
    #[builder(into)]
    pub r#hotlink_protection: Option<bool>,
    /// Identifier of the action parameter to modify.
    #[builder(into)]
    pub r#id: Option<String>,
    #[builder(into)]
    pub r#increment: Option<i32>,
    /// List of properties to configure WAF payload logging.
    #[builder(into)]
    pub r#matched_data: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersMatchedData>>,
    /// Turn on or off Cloudflare Mirage of the Cloudflare Speed app.
    #[builder(into)]
    pub r#mirage: Option<bool>,
    /// Turn on or off the Cloudflare Opportunistic Encryption feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[builder(into)]
    pub r#opportunistic_encryption: Option<bool>,
    /// List of properties to change request origin.
    #[builder(into)]
    pub r#origin: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersOrigin>>,
    /// Sets a more compliant mode for parsing Cache Control headers
    #[builder(into)]
    pub r#origin_cache_control: Option<bool>,
    /// Pass-through error page for origin.
    #[builder(into)]
    pub r#origin_error_page_passthru: Option<bool>,
    /// List of override configurations to apply to the ruleset.
    #[builder(into)]
    pub r#overrides: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersOverrides>>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`
    #[builder(into)]
    pub r#phases: Option<Vec<String>>,
    /// Apply options from the Polish feature of the Cloudflare Speed app.
    #[builder(into)]
    pub r#polish: Option<String>,
    /// Products to target with the actions. Available values: `bic`, `hot`, `ratelimit`, `securityLevel`, `uablock`, `waf`, `zonelockdown`
    #[builder(into)]
    pub r#products: Option<Vec<String>>,
    /// Sets the timeout value for reading content from an origin server.
    #[builder(into)]
    pub r#read_timeout: Option<i32>,
    /// List of request headers to include as part of custom fields logging, in lowercase.
    #[builder(into)]
    pub r#request_fields: Option<Vec<String>>,
    /// Respect strong ETags.
    #[builder(into)]
    pub r#respect_strong_etags: Option<bool>,
    /// List of response headers to include as part of custom fields logging, in lowercase.
    #[builder(into)]
    pub r#response_fields: Option<Vec<String>>,
    /// List of parameters that configure the response given to end users
    #[builder(into)]
    pub r#responses: Option<Vec<super::types::GetRulesetsRulesetRuleActionParametersResponse>>,
    /// Turn on or off Cloudflare Rocket Loader in the Cloudflare Speed app.
    #[builder(into)]
    pub r#rocket_loader: Option<bool>,
    /// Map of managed WAF rule ID to comma-delimited string of ruleset rule IDs. Example: `rules = { "efb7b8c949ac4650a09736fc376e9aee" = "5de7edfa648c4d6891dc3e7f84534ffa,e3a567afc347477d9702d9047e97d760" }`
    #[builder(into)]
    pub r#rules: Option<std::collections::HashMap<String, String>>,
    /// Which ruleset ID to target.
    #[builder(into)]
    pub r#ruleset: Option<String>,
    /// List of managed WAF rule IDs to target. Only valid when the `"action"` is set to skip
    #[builder(into)]
    pub r#rulesets: Option<Vec<String>>,
    /// Control options for the Security Level feature from the Security app.
    #[builder(into)]
    pub r#security_level: Option<String>,
    /// List of serve stale parameters to apply to the request.
    #[builder(into)]
    pub r#serve_stale: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersServeStale>>,
    /// Turn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app.
    #[builder(into)]
    pub r#server_side_excludes: Option<bool>,
    /// List of properties to manange Server Name Indication.
    #[builder(into)]
    pub r#sni: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersSni>>,
    /// Control options for the SSL feature of the Edge Certificates tab in the Cloudflare SSL/TLS app.
    #[builder(into)]
    pub r#ssl: Option<String>,
    /// HTTP status code of the custom error response
    #[builder(into)]
    pub r#status_code: Option<i32>,
    /// Turn on or off the SXG feature.
    #[builder(into)]
    pub r#sxg: Option<bool>,
    /// List of URI properties to configure for the ruleset rule when performing URL rewrite transformations.
    #[builder(into)]
    pub r#uri: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersUri>>,
    /// Version of the ruleset to deploy.
    #[builder(into)]
    pub r#version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRulesetsRulesetRuleActionParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "additionalCacheablePorts",
                    &self.r#additional_cacheable_ports,
                ),
                to_pulumi_object_field(
                    "automaticHttpsRewrites",
                    &self.r#automatic_https_rewrites,
                ),
                to_pulumi_object_field(
                    "autominifies",
                    &self.r#autominifies,
                ),
                to_pulumi_object_field(
                    "bic",
                    &self.r#bic,
                ),
                to_pulumi_object_field(
                    "browserTtl",
                    &self.r#browser_ttl,
                ),
                to_pulumi_object_field(
                    "cache",
                    &self.r#cache,
                ),
                to_pulumi_object_field(
                    "cacheKey",
                    &self.r#cache_key,
                ),
                to_pulumi_object_field(
                    "cacheReserve",
                    &self.r#cache_reserve,
                ),
                to_pulumi_object_field(
                    "content",
                    &self.r#content,
                ),
                to_pulumi_object_field(
                    "contentType",
                    &self.r#content_type,
                ),
                to_pulumi_object_field(
                    "cookieFields",
                    &self.r#cookie_fields,
                ),
                to_pulumi_object_field(
                    "disableApps",
                    &self.r#disable_apps,
                ),
                to_pulumi_object_field(
                    "disableRailgun",
                    &self.r#disable_railgun,
                ),
                to_pulumi_object_field(
                    "disableZaraz",
                    &self.r#disable_zaraz,
                ),
                to_pulumi_object_field(
                    "edgeTtl",
                    &self.r#edge_ttl,
                ),
                to_pulumi_object_field(
                    "emailObfuscation",
                    &self.r#email_obfuscation,
                ),
                to_pulumi_object_field(
                    "fromList",
                    &self.r#from_list,
                ),
                to_pulumi_object_field(
                    "fromValue",
                    &self.r#from_value,
                ),
                to_pulumi_object_field(
                    "headers",
                    &self.r#headers,
                ),
                to_pulumi_object_field(
                    "hostHeader",
                    &self.r#host_header,
                ),
                to_pulumi_object_field(
                    "hotlinkProtection",
                    &self.r#hotlink_protection,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "increment",
                    &self.r#increment,
                ),
                to_pulumi_object_field(
                    "matchedData",
                    &self.r#matched_data,
                ),
                to_pulumi_object_field(
                    "mirage",
                    &self.r#mirage,
                ),
                to_pulumi_object_field(
                    "opportunisticEncryption",
                    &self.r#opportunistic_encryption,
                ),
                to_pulumi_object_field(
                    "origin",
                    &self.r#origin,
                ),
                to_pulumi_object_field(
                    "originCacheControl",
                    &self.r#origin_cache_control,
                ),
                to_pulumi_object_field(
                    "originErrorPagePassthru",
                    &self.r#origin_error_page_passthru,
                ),
                to_pulumi_object_field(
                    "overrides",
                    &self.r#overrides,
                ),
                to_pulumi_object_field(
                    "phases",
                    &self.r#phases,
                ),
                to_pulumi_object_field(
                    "polish",
                    &self.r#polish,
                ),
                to_pulumi_object_field(
                    "products",
                    &self.r#products,
                ),
                to_pulumi_object_field(
                    "readTimeout",
                    &self.r#read_timeout,
                ),
                to_pulumi_object_field(
                    "requestFields",
                    &self.r#request_fields,
                ),
                to_pulumi_object_field(
                    "respectStrongEtags",
                    &self.r#respect_strong_etags,
                ),
                to_pulumi_object_field(
                    "responseFields",
                    &self.r#response_fields,
                ),
                to_pulumi_object_field(
                    "responses",
                    &self.r#responses,
                ),
                to_pulumi_object_field(
                    "rocketLoader",
                    &self.r#rocket_loader,
                ),
                to_pulumi_object_field(
                    "rules",
                    &self.r#rules,
                ),
                to_pulumi_object_field(
                    "ruleset",
                    &self.r#ruleset,
                ),
                to_pulumi_object_field(
                    "rulesets",
                    &self.r#rulesets,
                ),
                to_pulumi_object_field(
                    "securityLevel",
                    &self.r#security_level,
                ),
                to_pulumi_object_field(
                    "serveStale",
                    &self.r#serve_stale,
                ),
                to_pulumi_object_field(
                    "serverSideExcludes",
                    &self.r#server_side_excludes,
                ),
                to_pulumi_object_field(
                    "sni",
                    &self.r#sni,
                ),
                to_pulumi_object_field(
                    "ssl",
                    &self.r#ssl,
                ),
                to_pulumi_object_field(
                    "statusCode",
                    &self.r#status_code,
                ),
                to_pulumi_object_field(
                    "sxg",
                    &self.r#sxg,
                ),
                to_pulumi_object_field(
                    "uri",
                    &self.r#uri,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRulesetsRulesetRuleActionParameters {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#additional_cacheable_ports: {
                        let field_value = match fields_map.get("additionalCacheablePorts") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalCacheablePorts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#automatic_https_rewrites: {
                        let field_value = match fields_map.get("automaticHttpsRewrites") {
                            Some(value) => value,
                            None => bail!("Missing field 'automaticHttpsRewrites' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autominifies: {
                        let field_value = match fields_map.get("autominifies") {
                            Some(value) => value,
                            None => bail!("Missing field 'autominifies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bic: {
                        let field_value = match fields_map.get("bic") {
                            Some(value) => value,
                            None => bail!("Missing field 'bic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#browser_ttl: {
                        let field_value = match fields_map.get("browserTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'browserTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache: {
                        let field_value = match fields_map.get("cache") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_key: {
                        let field_value = match fields_map.get("cacheKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_reserve: {
                        let field_value = match fields_map.get("cacheReserve") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheReserve' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content: {
                        let field_value = match fields_map.get("content") {
                            Some(value) => value,
                            None => bail!("Missing field 'content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_type: {
                        let field_value = match fields_map.get("contentType") {
                            Some(value) => value,
                            None => bail!("Missing field 'contentType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookie_fields: {
                        let field_value = match fields_map.get("cookieFields") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookieFields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_apps: {
                        let field_value = match fields_map.get("disableApps") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableApps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_railgun: {
                        let field_value = match fields_map.get("disableRailgun") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableRailgun' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_zaraz: {
                        let field_value = match fields_map.get("disableZaraz") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableZaraz' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edge_ttl: {
                        let field_value = match fields_map.get("edgeTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'edgeTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_obfuscation: {
                        let field_value = match fields_map.get("emailObfuscation") {
                            Some(value) => value,
                            None => bail!("Missing field 'emailObfuscation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_list: {
                        let field_value = match fields_map.get("fromList") {
                            Some(value) => value,
                            None => bail!("Missing field 'fromList' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_value: {
                        let field_value = match fields_map.get("fromValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'fromValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_header: {
                        let field_value = match fields_map.get("hostHeader") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostHeader' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hotlink_protection: {
                        let field_value = match fields_map.get("hotlinkProtection") {
                            Some(value) => value,
                            None => bail!("Missing field 'hotlinkProtection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#increment: {
                        let field_value = match fields_map.get("increment") {
                            Some(value) => value,
                            None => bail!("Missing field 'increment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matched_data: {
                        let field_value = match fields_map.get("matchedData") {
                            Some(value) => value,
                            None => bail!("Missing field 'matchedData' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mirage: {
                        let field_value = match fields_map.get("mirage") {
                            Some(value) => value,
                            None => bail!("Missing field 'mirage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#opportunistic_encryption: {
                        let field_value = match fields_map.get("opportunisticEncryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'opportunisticEncryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin: {
                        let field_value = match fields_map.get("origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_cache_control: {
                        let field_value = match fields_map.get("originCacheControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'originCacheControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_error_page_passthru: {
                        let field_value = match fields_map.get("originErrorPagePassthru") {
                            Some(value) => value,
                            None => bail!("Missing field 'originErrorPagePassthru' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#overrides: {
                        let field_value = match fields_map.get("overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phases: {
                        let field_value = match fields_map.get("phases") {
                            Some(value) => value,
                            None => bail!("Missing field 'phases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#polish: {
                        let field_value = match fields_map.get("polish") {
                            Some(value) => value,
                            None => bail!("Missing field 'polish' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#products: {
                        let field_value = match fields_map.get("products") {
                            Some(value) => value,
                            None => bail!("Missing field 'products' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_timeout: {
                        let field_value = match fields_map.get("readTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'readTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_fields: {
                        let field_value = match fields_map.get("requestFields") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestFields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#respect_strong_etags: {
                        let field_value = match fields_map.get("respectStrongEtags") {
                            Some(value) => value,
                            None => bail!("Missing field 'respectStrongEtags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_fields: {
                        let field_value = match fields_map.get("responseFields") {
                            Some(value) => value,
                            None => bail!("Missing field 'responseFields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#responses: {
                        let field_value = match fields_map.get("responses") {
                            Some(value) => value,
                            None => bail!("Missing field 'responses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rocket_loader: {
                        let field_value = match fields_map.get("rocketLoader") {
                            Some(value) => value,
                            None => bail!("Missing field 'rocketLoader' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rules: {
                        let field_value = match fields_map.get("rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ruleset: {
                        let field_value = match fields_map.get("ruleset") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rulesets: {
                        let field_value = match fields_map.get("rulesets") {
                            Some(value) => value,
                            None => bail!("Missing field 'rulesets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_level: {
                        let field_value = match fields_map.get("securityLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serve_stale: {
                        let field_value = match fields_map.get("serveStale") {
                            Some(value) => value,
                            None => bail!("Missing field 'serveStale' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_side_excludes: {
                        let field_value = match fields_map.get("serverSideExcludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverSideExcludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sni: {
                        let field_value = match fields_map.get("sni") {
                            Some(value) => value,
                            None => bail!("Missing field 'sni' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl: {
                        let field_value = match fields_map.get("ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status_code: {
                        let field_value = match fields_map.get("statusCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'statusCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sxg: {
                        let field_value = match fields_map.get("sxg") {
                            Some(value) => value,
                            None => bail!("Missing field 'sxg' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri: {
                        let field_value = match fields_map.get("uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
