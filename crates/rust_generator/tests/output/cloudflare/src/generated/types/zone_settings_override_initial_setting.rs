#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZoneSettingsOverrideInitialSetting {
    #[builder(into)]
    #[serde(rename = "alwaysOnline")]
    pub r#always_online: Option<String>,
    #[builder(into)]
    #[serde(rename = "alwaysUseHttps")]
    pub r#always_use_https: Option<String>,
    #[builder(into)]
    #[serde(rename = "automaticHttpsRewrites")]
    pub r#automatic_https_rewrites: Option<String>,
    #[builder(into)]
    #[serde(rename = "binaryAst")]
    pub r#binary_ast: Option<String>,
    #[builder(into)]
    #[serde(rename = "brotli")]
    pub r#brotli: Option<String>,
    #[builder(into)]
    #[serde(rename = "browserCacheTtl")]
    pub r#browser_cache_ttl: Option<i32>,
    #[builder(into)]
    #[serde(rename = "browserCheck")]
    pub r#browser_check: Option<String>,
    #[builder(into)]
    #[serde(rename = "cacheLevel")]
    pub r#cache_level: Option<String>,
    #[builder(into)]
    #[serde(rename = "challengeTtl")]
    pub r#challenge_ttl: Option<i32>,
    #[builder(into)]
    #[serde(rename = "ciphers")]
    pub r#ciphers: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "cnameFlattening")]
    pub r#cname_flattening: Option<String>,
    #[builder(into)]
    #[serde(rename = "developmentMode")]
    pub r#development_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Option<String>,
    #[builder(into)]
    #[serde(rename = "emailObfuscation")]
    pub r#email_obfuscation: Option<String>,
    #[builder(into)]
    #[serde(rename = "filterLogsToCloudflare")]
    pub r#filter_logs_to_cloudflare: Option<String>,
    #[builder(into)]
    #[serde(rename = "fonts")]
    pub r#fonts: Option<String>,
    #[builder(into)]
    #[serde(rename = "h2Prioritization")]
    pub r#h_2_prioritization: Option<String>,
    #[builder(into)]
    #[serde(rename = "hotlinkProtection")]
    pub r#hotlink_protection: Option<String>,
    #[builder(into)]
    #[serde(rename = "http2")]
    pub r#http_2: Option<String>,
    #[builder(into)]
    #[serde(rename = "http3")]
    pub r#http_3: Option<String>,
    #[builder(into)]
    #[serde(rename = "imageResizing")]
    pub r#image_resizing: Option<String>,
    #[builder(into)]
    #[serde(rename = "ipGeolocation")]
    pub r#ip_geolocation: Option<String>,
    #[builder(into)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Option<String>,
    #[builder(into)]
    #[serde(rename = "logToCloudflare")]
    pub r#log_to_cloudflare: Option<String>,
    #[builder(into)]
    #[serde(rename = "maxUpload")]
    pub r#max_upload: Option<i32>,
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Option<String>,
    #[builder(into)]
    #[serde(rename = "minify")]
    pub r#minify: Option<Box<super::types::ZoneSettingsOverrideInitialSettingMinify>>,
    #[builder(into)]
    #[serde(rename = "mirage")]
    pub r#mirage: Option<String>,
    #[builder(into)]
    #[serde(rename = "mobileRedirect")]
    pub r#mobile_redirect: Option<Box<super::types::ZoneSettingsOverrideInitialSettingMobileRedirect>>,
    #[builder(into)]
    #[serde(rename = "nel")]
    pub r#nel: Option<Box<super::types::ZoneSettingsOverrideInitialSettingNel>>,
    #[builder(into)]
    #[serde(rename = "opportunisticEncryption")]
    pub r#opportunistic_encryption: Option<String>,
    #[builder(into)]
    #[serde(rename = "opportunisticOnion")]
    pub r#opportunistic_onion: Option<String>,
    #[builder(into)]
    #[serde(rename = "orangeToOrange")]
    pub r#orange_to_orange: Option<String>,
    #[builder(into)]
    #[serde(rename = "originErrorPagePassThru")]
    pub r#origin_error_page_pass_thru: Option<String>,
    #[builder(into)]
    #[serde(rename = "originMaxHttpVersion")]
    pub r#origin_max_http_version: Option<String>,
    #[builder(into)]
    #[serde(rename = "polish")]
    pub r#polish: Option<String>,
    #[builder(into)]
    #[serde(rename = "prefetchPreload")]
    pub r#prefetch_preload: Option<String>,
    #[builder(into)]
    #[serde(rename = "privacyPass")]
    pub r#privacy_pass: Option<String>,
    #[builder(into)]
    #[serde(rename = "proxyReadTimeout")]
    pub r#proxy_read_timeout: Option<String>,
    #[builder(into)]
    #[serde(rename = "pseudoIpv4")]
    pub r#pseudo_ipv_4: Option<String>,
    #[builder(into)]
    #[serde(rename = "replaceInsecureJs")]
    pub r#replace_insecure_js: Option<String>,
    #[builder(into)]
    #[serde(rename = "responseBuffering")]
    pub r#response_buffering: Option<String>,
    #[builder(into)]
    #[serde(rename = "rocketLoader")]
    pub r#rocket_loader: Option<String>,
    #[builder(into)]
    #[serde(rename = "securityHeader")]
    pub r#security_header: Option<Box<super::types::ZoneSettingsOverrideInitialSettingSecurityHeader>>,
    #[builder(into)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Option<String>,
    #[builder(into)]
    #[serde(rename = "serverSideExclude")]
    pub r#server_side_exclude: Option<String>,
    #[builder(into)]
    #[serde(rename = "sortQueryStringForCache")]
    pub r#sort_query_string_for_cache: Option<String>,
    #[builder(into)]
    #[serde(rename = "speedBrain")]
    pub r#speed_brain: Option<String>,
    #[builder(into)]
    #[serde(rename = "ssl")]
    pub r#ssl: Option<String>,
    #[builder(into)]
    #[serde(rename = "tls12Only")]
    pub r#tls_12_only: Option<String>,
    #[builder(into)]
    #[serde(rename = "tls13")]
    pub r#tls_13: Option<String>,
    #[builder(into)]
    #[serde(rename = "tlsClientAuth")]
    pub r#tls_client_auth: Option<String>,
    #[builder(into)]
    #[serde(rename = "trueClientIpHeader")]
    pub r#true_client_ip_header: Option<String>,
    #[builder(into)]
    #[serde(rename = "universalSsl")]
    pub r#universal_ssl: Option<String>,
    #[builder(into)]
    #[serde(rename = "visitorIp")]
    pub r#visitor_ip: Option<String>,
    #[builder(into)]
    #[serde(rename = "waf")]
    pub r#waf: Option<String>,
    #[builder(into)]
    #[serde(rename = "webp")]
    pub r#webp: Option<String>,
    #[builder(into)]
    #[serde(rename = "websockets")]
    pub r#websockets: Option<String>,
    #[builder(into)]
    #[serde(rename = "zeroRtt")]
    pub r#zero_rtt: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZoneSettingsOverrideInitialSetting {
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
                "always_online".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#always_online,
                )
                .await,
            );
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
                "binary_ast".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#binary_ast,
                )
                .await,
            );
            map.insert(
                "brotli".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#brotli,
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
                "cache_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_level,
                )
                .await,
            );
            map.insert(
                "challenge_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#challenge_ttl,
                )
                .await,
            );
            map.insert(
                "ciphers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ciphers,
                )
                .await,
            );
            map.insert(
                "cname_flattening".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cname_flattening,
                )
                .await,
            );
            map.insert(
                "development_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#development_mode,
                )
                .await,
            );
            map.insert(
                "early_hints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#early_hints,
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
                "filter_logs_to_cloudflare".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter_logs_to_cloudflare,
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
                "h_2_prioritization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#h_2_prioritization,
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
                "http_2".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_2,
                )
                .await,
            );
            map.insert(
                "http_3".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_3,
                )
                .await,
            );
            map.insert(
                "image_resizing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_resizing,
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
                "ipv_6".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6,
                )
                .await,
            );
            map.insert(
                "log_to_cloudflare".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_to_cloudflare,
                )
                .await,
            );
            map.insert(
                "max_upload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_upload,
                )
                .await,
            );
            map.insert(
                "min_tls_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_tls_version,
                )
                .await,
            );
            map.insert(
                "minify".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minify,
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
                "mobile_redirect".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mobile_redirect,
                )
                .await,
            );
            map.insert(
                "nel".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nel,
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
                "opportunistic_onion".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#opportunistic_onion,
                )
                .await,
            );
            map.insert(
                "orange_to_orange".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#orange_to_orange,
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
                "origin_max_http_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origin_max_http_version,
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
                "prefetch_preload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefetch_preload,
                )
                .await,
            );
            map.insert(
                "privacy_pass".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#privacy_pass,
                )
                .await,
            );
            map.insert(
                "proxy_read_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#proxy_read_timeout,
                )
                .await,
            );
            map.insert(
                "pseudo_ipv_4".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pseudo_ipv_4,
                )
                .await,
            );
            map.insert(
                "replace_insecure_js".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#replace_insecure_js,
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
                "security_header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_header,
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
                "speed_brain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#speed_brain,
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
                "tls_12_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tls_12_only,
                )
                .await,
            );
            map.insert(
                "tls_13".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tls_13,
                )
                .await,
            );
            map.insert(
                "tls_client_auth".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tls_client_auth,
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
                "universal_ssl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#universal_ssl,
                )
                .await,
            );
            map.insert(
                "visitor_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#visitor_ip,
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
            map.insert(
                "webp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#webp,
                )
                .await,
            );
            map.insert(
                "websockets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#websockets,
                )
                .await,
            );
            map.insert(
                "zero_rtt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zero_rtt,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZoneSettingsOverrideInitialSetting {
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
                    r#always_online: {
                        let field_value = match fields_map.get("always_online") {
                            Some(value) => value,
                            None => bail!("Missing field 'always_online' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#binary_ast: {
                        let field_value = match fields_map.get("binary_ast") {
                            Some(value) => value,
                            None => bail!("Missing field 'binary_ast' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#brotli: {
                        let field_value = match fields_map.get("brotli") {
                            Some(value) => value,
                            None => bail!("Missing field 'brotli' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#cache_level: {
                        let field_value = match fields_map.get("cache_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#challenge_ttl: {
                        let field_value = match fields_map.get("challenge_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'challenge_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ciphers: {
                        let field_value = match fields_map.get("ciphers") {
                            Some(value) => value,
                            None => bail!("Missing field 'ciphers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cname_flattening: {
                        let field_value = match fields_map.get("cname_flattening") {
                            Some(value) => value,
                            None => bail!("Missing field 'cname_flattening' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#development_mode: {
                        let field_value = match fields_map.get("development_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'development_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#early_hints: {
                        let field_value = match fields_map.get("early_hints") {
                            Some(value) => value,
                            None => bail!("Missing field 'early_hints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#filter_logs_to_cloudflare: {
                        let field_value = match fields_map.get("filter_logs_to_cloudflare") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_logs_to_cloudflare' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#h_2_prioritization: {
                        let field_value = match fields_map.get("h_2_prioritization") {
                            Some(value) => value,
                            None => bail!("Missing field 'h_2_prioritization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#http_2: {
                        let field_value = match fields_map.get("http_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_3: {
                        let field_value = match fields_map.get("http_3") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_resizing: {
                        let field_value = match fields_map.get("image_resizing") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_resizing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ipv_6: {
                        let field_value = match fields_map.get("ipv_6") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_to_cloudflare: {
                        let field_value = match fields_map.get("log_to_cloudflare") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_to_cloudflare' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_upload: {
                        let field_value = match fields_map.get("max_upload") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_upload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_tls_version: {
                        let field_value = match fields_map.get("min_tls_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_tls_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minify: {
                        let field_value = match fields_map.get("minify") {
                            Some(value) => value,
                            None => bail!("Missing field 'minify' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#mobile_redirect: {
                        let field_value = match fields_map.get("mobile_redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'mobile_redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nel: {
                        let field_value = match fields_map.get("nel") {
                            Some(value) => value,
                            None => bail!("Missing field 'nel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#opportunistic_onion: {
                        let field_value = match fields_map.get("opportunistic_onion") {
                            Some(value) => value,
                            None => bail!("Missing field 'opportunistic_onion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#orange_to_orange: {
                        let field_value = match fields_map.get("orange_to_orange") {
                            Some(value) => value,
                            None => bail!("Missing field 'orange_to_orange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#origin_max_http_version: {
                        let field_value = match fields_map.get("origin_max_http_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_max_http_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#prefetch_preload: {
                        let field_value = match fields_map.get("prefetch_preload") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefetch_preload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#privacy_pass: {
                        let field_value = match fields_map.get("privacy_pass") {
                            Some(value) => value,
                            None => bail!("Missing field 'privacy_pass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_read_timeout: {
                        let field_value = match fields_map.get("proxy_read_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_read_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pseudo_ipv_4: {
                        let field_value = match fields_map.get("pseudo_ipv_4") {
                            Some(value) => value,
                            None => bail!("Missing field 'pseudo_ipv_4' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replace_insecure_js: {
                        let field_value = match fields_map.get("replace_insecure_js") {
                            Some(value) => value,
                            None => bail!("Missing field 'replace_insecure_js' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#security_header: {
                        let field_value = match fields_map.get("security_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#speed_brain: {
                        let field_value = match fields_map.get("speed_brain") {
                            Some(value) => value,
                            None => bail!("Missing field 'speed_brain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tls_12_only: {
                        let field_value = match fields_map.get("tls_12_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_12_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_13: {
                        let field_value = match fields_map.get("tls_13") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_13' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_client_auth: {
                        let field_value = match fields_map.get("tls_client_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_client_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#universal_ssl: {
                        let field_value = match fields_map.get("universal_ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'universal_ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#visitor_ip: {
                        let field_value = match fields_map.get("visitor_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'visitor_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#webp: {
                        let field_value = match fields_map.get("webp") {
                            Some(value) => value,
                            None => bail!("Missing field 'webp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#websockets: {
                        let field_value = match fields_map.get("websockets") {
                            Some(value) => value,
                            None => bail!("Missing field 'websockets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zero_rtt: {
                        let field_value = match fields_map.get("zero_rtt") {
                            Some(value) => value,
                            None => bail!("Missing field 'zero_rtt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
