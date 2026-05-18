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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "always_online",
                    &self.r#always_online,
                ),
                to_pulumi_object_field(
                    "always_use_https",
                    &self.r#always_use_https,
                ),
                to_pulumi_object_field(
                    "automatic_https_rewrites",
                    &self.r#automatic_https_rewrites,
                ),
                to_pulumi_object_field(
                    "binary_ast",
                    &self.r#binary_ast,
                ),
                to_pulumi_object_field(
                    "brotli",
                    &self.r#brotli,
                ),
                to_pulumi_object_field(
                    "browser_cache_ttl",
                    &self.r#browser_cache_ttl,
                ),
                to_pulumi_object_field(
                    "browser_check",
                    &self.r#browser_check,
                ),
                to_pulumi_object_field(
                    "cache_level",
                    &self.r#cache_level,
                ),
                to_pulumi_object_field(
                    "challenge_ttl",
                    &self.r#challenge_ttl,
                ),
                to_pulumi_object_field(
                    "ciphers",
                    &self.r#ciphers,
                ),
                to_pulumi_object_field(
                    "cname_flattening",
                    &self.r#cname_flattening,
                ),
                to_pulumi_object_field(
                    "development_mode",
                    &self.r#development_mode,
                ),
                to_pulumi_object_field(
                    "early_hints",
                    &self.r#early_hints,
                ),
                to_pulumi_object_field(
                    "email_obfuscation",
                    &self.r#email_obfuscation,
                ),
                to_pulumi_object_field(
                    "filter_logs_to_cloudflare",
                    &self.r#filter_logs_to_cloudflare,
                ),
                to_pulumi_object_field(
                    "fonts",
                    &self.r#fonts,
                ),
                to_pulumi_object_field(
                    "h_2_prioritization",
                    &self.r#h_2_prioritization,
                ),
                to_pulumi_object_field(
                    "hotlink_protection",
                    &self.r#hotlink_protection,
                ),
                to_pulumi_object_field(
                    "http_2",
                    &self.r#http_2,
                ),
                to_pulumi_object_field(
                    "http_3",
                    &self.r#http_3,
                ),
                to_pulumi_object_field(
                    "image_resizing",
                    &self.r#image_resizing,
                ),
                to_pulumi_object_field(
                    "ip_geolocation",
                    &self.r#ip_geolocation,
                ),
                to_pulumi_object_field(
                    "ipv_6",
                    &self.r#ipv_6,
                ),
                to_pulumi_object_field(
                    "log_to_cloudflare",
                    &self.r#log_to_cloudflare,
                ),
                to_pulumi_object_field(
                    "max_upload",
                    &self.r#max_upload,
                ),
                to_pulumi_object_field(
                    "min_tls_version",
                    &self.r#min_tls_version,
                ),
                to_pulumi_object_field(
                    "minify",
                    &self.r#minify,
                ),
                to_pulumi_object_field(
                    "mirage",
                    &self.r#mirage,
                ),
                to_pulumi_object_field(
                    "mobile_redirect",
                    &self.r#mobile_redirect,
                ),
                to_pulumi_object_field(
                    "nel",
                    &self.r#nel,
                ),
                to_pulumi_object_field(
                    "opportunistic_encryption",
                    &self.r#opportunistic_encryption,
                ),
                to_pulumi_object_field(
                    "opportunistic_onion",
                    &self.r#opportunistic_onion,
                ),
                to_pulumi_object_field(
                    "orange_to_orange",
                    &self.r#orange_to_orange,
                ),
                to_pulumi_object_field(
                    "origin_error_page_pass_thru",
                    &self.r#origin_error_page_pass_thru,
                ),
                to_pulumi_object_field(
                    "origin_max_http_version",
                    &self.r#origin_max_http_version,
                ),
                to_pulumi_object_field(
                    "polish",
                    &self.r#polish,
                ),
                to_pulumi_object_field(
                    "prefetch_preload",
                    &self.r#prefetch_preload,
                ),
                to_pulumi_object_field(
                    "privacy_pass",
                    &self.r#privacy_pass,
                ),
                to_pulumi_object_field(
                    "proxy_read_timeout",
                    &self.r#proxy_read_timeout,
                ),
                to_pulumi_object_field(
                    "pseudo_ipv_4",
                    &self.r#pseudo_ipv_4,
                ),
                to_pulumi_object_field(
                    "replace_insecure_js",
                    &self.r#replace_insecure_js,
                ),
                to_pulumi_object_field(
                    "response_buffering",
                    &self.r#response_buffering,
                ),
                to_pulumi_object_field(
                    "rocket_loader",
                    &self.r#rocket_loader,
                ),
                to_pulumi_object_field(
                    "security_header",
                    &self.r#security_header,
                ),
                to_pulumi_object_field(
                    "security_level",
                    &self.r#security_level,
                ),
                to_pulumi_object_field(
                    "server_side_exclude",
                    &self.r#server_side_exclude,
                ),
                to_pulumi_object_field(
                    "sort_query_string_for_cache",
                    &self.r#sort_query_string_for_cache,
                ),
                to_pulumi_object_field(
                    "speed_brain",
                    &self.r#speed_brain,
                ),
                to_pulumi_object_field(
                    "ssl",
                    &self.r#ssl,
                ),
                to_pulumi_object_field(
                    "tls_12_only",
                    &self.r#tls_12_only,
                ),
                to_pulumi_object_field(
                    "tls_13",
                    &self.r#tls_13,
                ),
                to_pulumi_object_field(
                    "tls_client_auth",
                    &self.r#tls_client_auth,
                ),
                to_pulumi_object_field(
                    "true_client_ip_header",
                    &self.r#true_client_ip_header,
                ),
                to_pulumi_object_field(
                    "universal_ssl",
                    &self.r#universal_ssl,
                ),
                to_pulumi_object_field(
                    "visitor_ip",
                    &self.r#visitor_ip,
                ),
                to_pulumi_object_field(
                    "waf",
                    &self.r#waf,
                ),
                to_pulumi_object_field(
                    "webp",
                    &self.r#webp,
                ),
                to_pulumi_object_field(
                    "websockets",
                    &self.r#websockets,
                ),
                to_pulumi_object_field(
                    "zero_rtt",
                    &self.r#zero_rtt,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
