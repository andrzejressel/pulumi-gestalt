#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
    pub r#browser_ttl: Option<Box<super::types::RulesetRuleActionParametersBrowserTtl>>,
    /// Whether to cache if expression matches.
    #[builder(into)]
    #[serde(rename = "cache")]
    pub r#cache: Option<bool>,
    /// List of cache key parameters to apply to the request.
    #[builder(into)]
    #[serde(rename = "cacheKey")]
    pub r#cache_key: Option<Box<super::types::RulesetRuleActionParametersCacheKey>>,
    /// List of cache reserve parameters to apply to the request.
    #[builder(into)]
    #[serde(rename = "cacheReserve")]
    pub r#cache_reserve: Option<Box<super::types::RulesetRuleActionParametersCacheReserve>>,
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
    pub r#edge_ttl: Option<Box<super::types::RulesetRuleActionParametersEdgeTtl>>,
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
    pub r#from_list: Option<Box<super::types::RulesetRuleActionParametersFromList>>,
    /// Use a value to lookup information for the action.
    #[builder(into)]
    #[serde(rename = "fromValue")]
    pub r#from_value: Option<Box<super::types::RulesetRuleActionParametersFromValue>>,
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
    pub r#matched_data: Option<Box<super::types::RulesetRuleActionParametersMatchedData>>,
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
    pub r#origin: Option<Box<super::types::RulesetRuleActionParametersOrigin>>,
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
    pub r#overrides: Option<Box<super::types::RulesetRuleActionParametersOverrides>>,
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
    pub r#serve_stale: Option<Box<super::types::RulesetRuleActionParametersServeStale>>,
    /// Turn on or off the Server Side Excludes feature of the Cloudflare Scrape Shield app.
    #[builder(into)]
    #[serde(rename = "serverSideExcludes")]
    pub r#server_side_excludes: Option<bool>,
    /// List of properties to manange Server Name Indication.
    #[builder(into)]
    #[serde(rename = "sni")]
    pub r#sni: Option<Box<super::types::RulesetRuleActionParametersSni>>,
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
    pub r#uri: Option<Box<super::types::RulesetRuleActionParametersUri>>,
    /// Version of the ruleset to deploy.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RulesetRuleActionParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "additional_cacheable_ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_cacheable_ports,
                )
                .await,
            );
            map.insert(
                "algorithms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#algorithms,
                )
                .await,
            );
            map.insert(
                "automatic_https_rewrites".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#automatic_https_rewrites,
                )
                .await,
            );
            map.insert(
                "autominifies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autominifies,
                )
                .await,
            );
            map.insert(
                "bic".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bic,
                )
                .await,
            );
            map.insert(
                "browser_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#browser_ttl,
                )
                .await,
            );
            map.insert(
                "cache".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache,
                )
                .await,
            );
            map.insert(
                "cache_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_key,
                )
                .await,
            );
            map.insert(
                "cache_reserve".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_reserve,
                )
                .await,
            );
            map.insert(
                "content".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content,
                )
                .await,
            );
            map.insert(
                "content_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content_type,
                )
                .await,
            );
            map.insert(
                "cookie_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cookie_fields,
                )
                .await,
            );
            map.insert(
                "disable_apps".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_apps,
                )
                .await,
            );
            map.insert(
                "disable_railgun".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_railgun,
                )
                .await,
            );
            map.insert(
                "disable_rum".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_rum,
                )
                .await,
            );
            map.insert(
                "disable_zaraz".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_zaraz,
                )
                .await,
            );
            map.insert(
                "edge_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#edge_ttl,
                )
                .await,
            );
            map.insert(
                "email_obfuscation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_obfuscation,
                )
                .await,
            );
            map.insert(
                "fonts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fonts,
                )
                .await,
            );
            map.insert(
                "from_list".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#from_list,
                )
                .await,
            );
            map.insert(
                "from_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#from_value,
                )
                .await,
            );
            map.insert(
                "headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers,
                )
                .await,
            );
            map.insert(
                "host_header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_header,
                )
                .await,
            );
            map.insert(
                "hotlink_protection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hotlink_protection,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "increment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#increment,
                )
                .await,
            );
            map.insert(
                "matched_data".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#matched_data,
                )
                .await,
            );
            map.insert(
                "mirage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mirage,
                )
                .await,
            );
            map.insert(
                "opportunistic_encryption".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#opportunistic_encryption,
                )
                .await,
            );
            map.insert(
                "origin".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origin,
                )
                .await,
            );
            map.insert(
                "origin_cache_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origin_cache_control,
                )
                .await,
            );
            map.insert(
                "origin_error_page_passthru".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origin_error_page_passthru,
                )
                .await,
            );
            map.insert(
                "overrides".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#overrides,
                )
                .await,
            );
            map.insert(
                "phases".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#phases,
                )
                .await,
            );
            map.insert(
                "polish".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#polish,
                )
                .await,
            );
            map.insert(
                "products".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#products,
                )
                .await,
            );
            map.insert(
                "read_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#read_timeout,
                )
                .await,
            );
            map.insert(
                "request_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_fields,
                )
                .await,
            );
            map.insert(
                "respect_strong_etags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#respect_strong_etags,
                )
                .await,
            );
            map.insert(
                "response_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response_fields,
                )
                .await,
            );
            map.insert(
                "responses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#responses,
                )
                .await,
            );
            map.insert(
                "rocket_loader".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rocket_loader,
                )
                .await,
            );
            map.insert(
                "rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rules,
                )
                .await,
            );
            map.insert(
                "ruleset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ruleset,
                )
                .await,
            );
            map.insert(
                "rulesets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rulesets,
                )
                .await,
            );
            map.insert(
                "security_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_level,
                )
                .await,
            );
            map.insert(
                "serve_stale".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serve_stale,
                )
                .await,
            );
            map.insert(
                "server_side_excludes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_side_excludes,
                )
                .await,
            );
            map.insert(
                "sni".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sni,
                )
                .await,
            );
            map.insert(
                "ssl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl,
                )
                .await,
            );
            map.insert(
                "status_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status_code,
                )
                .await,
            );
            map.insert(
                "sxg".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sxg,
                )
                .await,
            );
            map.insert(
                "uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uri,
                )
                .await,
            );
            map.insert(
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RulesetRuleActionParameters {
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
                        let field_value = match fields_map.get("additional_cacheable_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_cacheable_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#algorithms: {
                        let field_value = match fields_map.get("algorithms") {
                            Some(value) => value,
                            None => bail!("Missing field 'algorithms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#automatic_https_rewrites: {
                        let field_value = match fields_map.get("automatic_https_rewrites") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic_https_rewrites' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("browser_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'browser_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("cache_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_reserve: {
                        let field_value = match fields_map.get("cache_reserve") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_reserve' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("content_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookie_fields: {
                        let field_value = match fields_map.get("cookie_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookie_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_apps: {
                        let field_value = match fields_map.get("disable_apps") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_apps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_railgun: {
                        let field_value = match fields_map.get("disable_railgun") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_railgun' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_rum: {
                        let field_value = match fields_map.get("disable_rum") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_rum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_zaraz: {
                        let field_value = match fields_map.get("disable_zaraz") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_zaraz' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edge_ttl: {
                        let field_value = match fields_map.get("edge_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_obfuscation: {
                        let field_value = match fields_map.get("email_obfuscation") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_obfuscation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fonts: {
                        let field_value = match fields_map.get("fonts") {
                            Some(value) => value,
                            None => bail!("Missing field 'fonts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_list: {
                        let field_value = match fields_map.get("from_list") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_list' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_value: {
                        let field_value = match fields_map.get("from_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("host_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hotlink_protection: {
                        let field_value = match fields_map.get("hotlink_protection") {
                            Some(value) => value,
                            None => bail!("Missing field 'hotlink_protection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("matched_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'matched_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("opportunistic_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'opportunistic_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("origin_cache_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_cache_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_error_page_passthru: {
                        let field_value = match fields_map.get("origin_error_page_passthru") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_error_page_passthru' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("read_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_fields: {
                        let field_value = match fields_map.get("request_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#respect_strong_etags: {
                        let field_value = match fields_map.get("respect_strong_etags") {
                            Some(value) => value,
                            None => bail!("Missing field 'respect_strong_etags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_fields: {
                        let field_value = match fields_map.get("response_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("rocket_loader") {
                            Some(value) => value,
                            None => bail!("Missing field 'rocket_loader' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("security_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serve_stale: {
                        let field_value = match fields_map.get("serve_stale") {
                            Some(value) => value,
                            None => bail!("Missing field 'serve_stale' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_side_excludes: {
                        let field_value = match fields_map.get("server_side_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_side_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("status_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'status_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
