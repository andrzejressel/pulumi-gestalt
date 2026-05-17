#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PageRuleActions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "always_use_https".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#always_use_https,
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
                "browser_cache_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#browser_cache_ttl,
                )
                .await,
            );
            map.insert(
                "browser_check".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#browser_check,
                )
                .await,
            );
            map.insert(
                "bypass_cache_on_cookie".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bypass_cache_on_cookie,
                )
                .await,
            );
            map.insert(
                "cache_by_device_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_by_device_type,
                )
                .await,
            );
            map.insert(
                "cache_deception_armor".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_deception_armor,
                )
                .await,
            );
            map.insert(
                "cache_key_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_key_fields,
                )
                .await,
            );
            map.insert(
                "cache_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_level,
                )
                .await,
            );
            map.insert(
                "cache_on_cookie".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_on_cookie,
                )
                .await,
            );
            map.insert(
                "cache_ttl_by_statuses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_ttl_by_statuses,
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
                "disable_performance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_performance,
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
                "disable_security".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_security,
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
                "edge_cache_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#edge_cache_ttl,
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
                "explicit_cache_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#explicit_cache_control,
                )
                .await,
            );
            map.insert(
                "forwarding_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forwarding_url,
                )
                .await,
            );
            map.insert(
                "host_header_override".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_header_override,
                )
                .await,
            );
            map.insert(
                "ip_geolocation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_geolocation,
                )
                .await,
            );
            map.insert(
                "minifies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minifies,
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
                "origin_error_page_pass_thru".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origin_error_page_pass_thru,
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
                "resolve_override".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resolve_override,
                )
                .await,
            );
            map.insert(
                "respect_strong_etag".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#respect_strong_etag,
                )
                .await,
            );
            map.insert(
                "response_buffering".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response_buffering,
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
                "security_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_level,
                )
                .await,
            );
            map.insert(
                "server_side_exclude".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_side_exclude,
                )
                .await,
            );
            map.insert(
                "sort_query_string_for_cache".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sort_query_string_for_cache,
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
                "true_client_ip_header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#true_client_ip_header,
                )
                .await,
            );
            map.insert(
                "waf".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#waf,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PageRuleActions {
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
                    r#always_use_https: {
                        let field_value = match fields_map.get("always_use_https") {
                            Some(value) => value,
                            None => bail!("Missing field 'always_use_https' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#browser_cache_ttl: {
                        let field_value = match fields_map.get("browser_cache_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'browser_cache_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#browser_check: {
                        let field_value = match fields_map.get("browser_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'browser_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bypass_cache_on_cookie: {
                        let field_value = match fields_map.get("bypass_cache_on_cookie") {
                            Some(value) => value,
                            None => bail!("Missing field 'bypass_cache_on_cookie' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_by_device_type: {
                        let field_value = match fields_map.get("cache_by_device_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_by_device_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_deception_armor: {
                        let field_value = match fields_map.get("cache_deception_armor") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_deception_armor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_key_fields: {
                        let field_value = match fields_map.get("cache_key_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_key_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_level: {
                        let field_value = match fields_map.get("cache_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_on_cookie: {
                        let field_value = match fields_map.get("cache_on_cookie") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_on_cookie' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_ttl_by_statuses: {
                        let field_value = match fields_map.get("cache_ttl_by_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_ttl_by_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#disable_performance: {
                        let field_value = match fields_map.get("disable_performance") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_performance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#disable_security: {
                        let field_value = match fields_map.get("disable_security") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_security' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#edge_cache_ttl: {
                        let field_value = match fields_map.get("edge_cache_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_cache_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#explicit_cache_control: {
                        let field_value = match fields_map.get("explicit_cache_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'explicit_cache_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forwarding_url: {
                        let field_value = match fields_map.get("forwarding_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarding_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_header_override: {
                        let field_value = match fields_map.get("host_header_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_header_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_geolocation: {
                        let field_value = match fields_map.get("ip_geolocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_geolocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minifies: {
                        let field_value = match fields_map.get("minifies") {
                            Some(value) => value,
                            None => bail!("Missing field 'minifies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#origin_error_page_pass_thru: {
                        let field_value = match fields_map.get("origin_error_page_pass_thru") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_error_page_pass_thru' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#resolve_override: {
                        let field_value = match fields_map.get("resolve_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'resolve_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#respect_strong_etag: {
                        let field_value = match fields_map.get("respect_strong_etag") {
                            Some(value) => value,
                            None => bail!("Missing field 'respect_strong_etag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_buffering: {
                        let field_value = match fields_map.get("response_buffering") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_buffering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#security_level: {
                        let field_value = match fields_map.get("security_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_side_exclude: {
                        let field_value = match fields_map.get("server_side_exclude") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_side_exclude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sort_query_string_for_cache: {
                        let field_value = match fields_map.get("sort_query_string_for_cache") {
                            Some(value) => value,
                            None => bail!("Missing field 'sort_query_string_for_cache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#true_client_ip_header: {
                        let field_value = match fields_map.get("true_client_ip_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'true_client_ip_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#waf: {
                        let field_value = match fields_map.get("waf") {
                            Some(value) => value,
                            None => bail!("Missing field 'waf' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
