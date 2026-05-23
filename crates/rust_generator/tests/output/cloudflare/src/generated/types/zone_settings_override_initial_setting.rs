#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZoneSettingsOverrideInitialSetting {
    #[builder(into)]
    pub r#always_online: Option<String>,
    #[builder(into)]
    pub r#always_use_https: Option<String>,
    #[builder(into)]
    pub r#automatic_https_rewrites: Option<String>,
    #[builder(into)]
    pub r#binary_ast: Option<String>,
    #[builder(into)]
    pub r#brotli: Option<String>,
    #[builder(into)]
    pub r#browser_cache_ttl: Option<i32>,
    #[builder(into)]
    pub r#browser_check: Option<String>,
    #[builder(into)]
    pub r#cache_level: Option<String>,
    #[builder(into)]
    pub r#challenge_ttl: Option<i32>,
    #[builder(into)]
    pub r#ciphers: Option<Vec<String>>,
    #[builder(into)]
    pub r#cname_flattening: Option<String>,
    #[builder(into)]
    pub r#development_mode: Option<String>,
    #[builder(into)]
    pub r#early_hints: Option<String>,
    #[builder(into)]
    pub r#email_obfuscation: Option<String>,
    #[builder(into)]
    pub r#filter_logs_to_cloudflare: Option<String>,
    #[builder(into)]
    pub r#fonts: Option<String>,
    #[builder(into)]
    pub r#h_2_prioritization: Option<String>,
    #[builder(into)]
    pub r#hotlink_protection: Option<String>,
    #[builder(into)]
    pub r#http_2: Option<String>,
    #[builder(into)]
    pub r#http_3: Option<String>,
    #[builder(into)]
    pub r#image_resizing: Option<String>,
    #[builder(into)]
    pub r#ip_geolocation: Option<String>,
    #[builder(into)]
    pub r#ipv_6: Option<String>,
    #[builder(into)]
    pub r#log_to_cloudflare: Option<String>,
    #[builder(into)]
    pub r#max_upload: Option<i32>,
    #[builder(into)]
    pub r#min_tls_version: Option<String>,
    #[builder(into)]
    pub r#minify: Option<Box<super::types::ZoneSettingsOverrideInitialSettingMinify>>,
    #[builder(into)]
    pub r#mirage: Option<String>,
    #[builder(into)]
    pub r#mobile_redirect: Option<Box<super::types::ZoneSettingsOverrideInitialSettingMobileRedirect>>,
    #[builder(into)]
    pub r#nel: Option<Box<super::types::ZoneSettingsOverrideInitialSettingNel>>,
    #[builder(into)]
    pub r#opportunistic_encryption: Option<String>,
    #[builder(into)]
    pub r#opportunistic_onion: Option<String>,
    #[builder(into)]
    pub r#orange_to_orange: Option<String>,
    #[builder(into)]
    pub r#origin_error_page_pass_thru: Option<String>,
    #[builder(into)]
    pub r#origin_max_http_version: Option<String>,
    #[builder(into)]
    pub r#polish: Option<String>,
    #[builder(into)]
    pub r#prefetch_preload: Option<String>,
    #[builder(into)]
    pub r#privacy_pass: Option<String>,
    #[builder(into)]
    pub r#proxy_read_timeout: Option<String>,
    #[builder(into)]
    pub r#pseudo_ipv_4: Option<String>,
    #[builder(into)]
    pub r#replace_insecure_js: Option<String>,
    #[builder(into)]
    pub r#response_buffering: Option<String>,
    #[builder(into)]
    pub r#rocket_loader: Option<String>,
    #[builder(into)]
    pub r#security_header: Option<Box<super::types::ZoneSettingsOverrideInitialSettingSecurityHeader>>,
    #[builder(into)]
    pub r#security_level: Option<String>,
    #[builder(into)]
    pub r#server_side_exclude: Option<String>,
    #[builder(into)]
    pub r#sort_query_string_for_cache: Option<String>,
    #[builder(into)]
    pub r#speed_brain: Option<String>,
    #[builder(into)]
    pub r#ssl: Option<String>,
    #[builder(into)]
    pub r#tls_12_only: Option<String>,
    #[builder(into)]
    pub r#tls_13: Option<String>,
    #[builder(into)]
    pub r#tls_client_auth: Option<String>,
    #[builder(into)]
    pub r#true_client_ip_header: Option<String>,
    #[builder(into)]
    pub r#universal_ssl: Option<String>,
    #[builder(into)]
    pub r#visitor_ip: Option<String>,
    #[builder(into)]
    pub r#waf: Option<String>,
    #[builder(into)]
    pub r#webp: Option<String>,
    #[builder(into)]
    pub r#websockets: Option<String>,
    #[builder(into)]
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
                    "alwaysOnline",
                    &self.r#always_online,
                ),
                to_pulumi_object_field(
                    "alwaysUseHttps",
                    &self.r#always_use_https,
                ),
                to_pulumi_object_field(
                    "automaticHttpsRewrites",
                    &self.r#automatic_https_rewrites,
                ),
                to_pulumi_object_field(
                    "binaryAst",
                    &self.r#binary_ast,
                ),
                to_pulumi_object_field(
                    "brotli",
                    &self.r#brotli,
                ),
                to_pulumi_object_field(
                    "browserCacheTtl",
                    &self.r#browser_cache_ttl,
                ),
                to_pulumi_object_field(
                    "browserCheck",
                    &self.r#browser_check,
                ),
                to_pulumi_object_field(
                    "cacheLevel",
                    &self.r#cache_level,
                ),
                to_pulumi_object_field(
                    "challengeTtl",
                    &self.r#challenge_ttl,
                ),
                to_pulumi_object_field(
                    "ciphers",
                    &self.r#ciphers,
                ),
                to_pulumi_object_field(
                    "cnameFlattening",
                    &self.r#cname_flattening,
                ),
                to_pulumi_object_field(
                    "developmentMode",
                    &self.r#development_mode,
                ),
                to_pulumi_object_field(
                    "earlyHints",
                    &self.r#early_hints,
                ),
                to_pulumi_object_field(
                    "emailObfuscation",
                    &self.r#email_obfuscation,
                ),
                to_pulumi_object_field(
                    "filterLogsToCloudflare",
                    &self.r#filter_logs_to_cloudflare,
                ),
                to_pulumi_object_field(
                    "fonts",
                    &self.r#fonts,
                ),
                to_pulumi_object_field(
                    "h2Prioritization",
                    &self.r#h_2_prioritization,
                ),
                to_pulumi_object_field(
                    "hotlinkProtection",
                    &self.r#hotlink_protection,
                ),
                to_pulumi_object_field(
                    "http2",
                    &self.r#http_2,
                ),
                to_pulumi_object_field(
                    "http3",
                    &self.r#http_3,
                ),
                to_pulumi_object_field(
                    "imageResizing",
                    &self.r#image_resizing,
                ),
                to_pulumi_object_field(
                    "ipGeolocation",
                    &self.r#ip_geolocation,
                ),
                to_pulumi_object_field(
                    "ipv6",
                    &self.r#ipv_6,
                ),
                to_pulumi_object_field(
                    "logToCloudflare",
                    &self.r#log_to_cloudflare,
                ),
                to_pulumi_object_field(
                    "maxUpload",
                    &self.r#max_upload,
                ),
                to_pulumi_object_field(
                    "minTlsVersion",
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
                    "mobileRedirect",
                    &self.r#mobile_redirect,
                ),
                to_pulumi_object_field(
                    "nel",
                    &self.r#nel,
                ),
                to_pulumi_object_field(
                    "opportunisticEncryption",
                    &self.r#opportunistic_encryption,
                ),
                to_pulumi_object_field(
                    "opportunisticOnion",
                    &self.r#opportunistic_onion,
                ),
                to_pulumi_object_field(
                    "orangeToOrange",
                    &self.r#orange_to_orange,
                ),
                to_pulumi_object_field(
                    "originErrorPagePassThru",
                    &self.r#origin_error_page_pass_thru,
                ),
                to_pulumi_object_field(
                    "originMaxHttpVersion",
                    &self.r#origin_max_http_version,
                ),
                to_pulumi_object_field(
                    "polish",
                    &self.r#polish,
                ),
                to_pulumi_object_field(
                    "prefetchPreload",
                    &self.r#prefetch_preload,
                ),
                to_pulumi_object_field(
                    "privacyPass",
                    &self.r#privacy_pass,
                ),
                to_pulumi_object_field(
                    "proxyReadTimeout",
                    &self.r#proxy_read_timeout,
                ),
                to_pulumi_object_field(
                    "pseudoIpv4",
                    &self.r#pseudo_ipv_4,
                ),
                to_pulumi_object_field(
                    "replaceInsecureJs",
                    &self.r#replace_insecure_js,
                ),
                to_pulumi_object_field(
                    "responseBuffering",
                    &self.r#response_buffering,
                ),
                to_pulumi_object_field(
                    "rocketLoader",
                    &self.r#rocket_loader,
                ),
                to_pulumi_object_field(
                    "securityHeader",
                    &self.r#security_header,
                ),
                to_pulumi_object_field(
                    "securityLevel",
                    &self.r#security_level,
                ),
                to_pulumi_object_field(
                    "serverSideExclude",
                    &self.r#server_side_exclude,
                ),
                to_pulumi_object_field(
                    "sortQueryStringForCache",
                    &self.r#sort_query_string_for_cache,
                ),
                to_pulumi_object_field(
                    "speedBrain",
                    &self.r#speed_brain,
                ),
                to_pulumi_object_field(
                    "ssl",
                    &self.r#ssl,
                ),
                to_pulumi_object_field(
                    "tls12Only",
                    &self.r#tls_12_only,
                ),
                to_pulumi_object_field(
                    "tls13",
                    &self.r#tls_13,
                ),
                to_pulumi_object_field(
                    "tlsClientAuth",
                    &self.r#tls_client_auth,
                ),
                to_pulumi_object_field(
                    "trueClientIpHeader",
                    &self.r#true_client_ip_header,
                ),
                to_pulumi_object_field(
                    "universalSsl",
                    &self.r#universal_ssl,
                ),
                to_pulumi_object_field(
                    "visitorIp",
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
                    "zeroRtt",
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
                        let field_value = match fields_map.get("alwaysOnline") {
                            Some(value) => value,
                            None => bail!("Missing field 'alwaysOnline' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#always_use_https: {
                        let field_value = match fields_map.get("alwaysUseHttps") {
                            Some(value) => value,
                            None => bail!("Missing field 'alwaysUseHttps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#binary_ast: {
                        let field_value = match fields_map.get("binaryAst") {
                            Some(value) => value,
                            None => bail!("Missing field 'binaryAst' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("browserCacheTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'browserCacheTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#browser_check: {
                        let field_value = match fields_map.get("browserCheck") {
                            Some(value) => value,
                            None => bail!("Missing field 'browserCheck' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_level: {
                        let field_value = match fields_map.get("cacheLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#challenge_ttl: {
                        let field_value = match fields_map.get("challengeTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'challengeTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("cnameFlattening") {
                            Some(value) => value,
                            None => bail!("Missing field 'cnameFlattening' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#development_mode: {
                        let field_value = match fields_map.get("developmentMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'developmentMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#early_hints: {
                        let field_value = match fields_map.get("earlyHints") {
                            Some(value) => value,
                            None => bail!("Missing field 'earlyHints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#filter_logs_to_cloudflare: {
                        let field_value = match fields_map.get("filterLogsToCloudflare") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterLogsToCloudflare' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("h2Prioritization") {
                            Some(value) => value,
                            None => bail!("Missing field 'h2Prioritization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#http_2: {
                        let field_value = match fields_map.get("http2") {
                            Some(value) => value,
                            None => bail!("Missing field 'http2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_3: {
                        let field_value = match fields_map.get("http3") {
                            Some(value) => value,
                            None => bail!("Missing field 'http3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_resizing: {
                        let field_value = match fields_map.get("imageResizing") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageResizing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_geolocation: {
                        let field_value = match fields_map.get("ipGeolocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipGeolocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6: {
                        let field_value = match fields_map.get("ipv6") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_to_cloudflare: {
                        let field_value = match fields_map.get("logToCloudflare") {
                            Some(value) => value,
                            None => bail!("Missing field 'logToCloudflare' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_upload: {
                        let field_value = match fields_map.get("maxUpload") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxUpload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_tls_version: {
                        let field_value = match fields_map.get("minTlsVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'minTlsVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("mobileRedirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'mobileRedirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("opportunisticEncryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'opportunisticEncryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#opportunistic_onion: {
                        let field_value = match fields_map.get("opportunisticOnion") {
                            Some(value) => value,
                            None => bail!("Missing field 'opportunisticOnion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#orange_to_orange: {
                        let field_value = match fields_map.get("orangeToOrange") {
                            Some(value) => value,
                            None => bail!("Missing field 'orangeToOrange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_error_page_pass_thru: {
                        let field_value = match fields_map.get("originErrorPagePassThru") {
                            Some(value) => value,
                            None => bail!("Missing field 'originErrorPagePassThru' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_max_http_version: {
                        let field_value = match fields_map.get("originMaxHttpVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'originMaxHttpVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("prefetchPreload") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefetchPreload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#privacy_pass: {
                        let field_value = match fields_map.get("privacyPass") {
                            Some(value) => value,
                            None => bail!("Missing field 'privacyPass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_read_timeout: {
                        let field_value = match fields_map.get("proxyReadTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxyReadTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pseudo_ipv_4: {
                        let field_value = match fields_map.get("pseudoIpv4") {
                            Some(value) => value,
                            None => bail!("Missing field 'pseudoIpv4' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replace_insecure_js: {
                        let field_value = match fields_map.get("replaceInsecureJs") {
                            Some(value) => value,
                            None => bail!("Missing field 'replaceInsecureJs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_buffering: {
                        let field_value = match fields_map.get("responseBuffering") {
                            Some(value) => value,
                            None => bail!("Missing field 'responseBuffering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#security_header: {
                        let field_value = match fields_map.get("securityHeader") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityHeader' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#server_side_exclude: {
                        let field_value = match fields_map.get("serverSideExclude") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverSideExclude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sort_query_string_for_cache: {
                        let field_value = match fields_map.get("sortQueryStringForCache") {
                            Some(value) => value,
                            None => bail!("Missing field 'sortQueryStringForCache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#speed_brain: {
                        let field_value = match fields_map.get("speedBrain") {
                            Some(value) => value,
                            None => bail!("Missing field 'speedBrain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("tls12Only") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls12Only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_13: {
                        let field_value = match fields_map.get("tls13") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls13' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_client_auth: {
                        let field_value = match fields_map.get("tlsClientAuth") {
                            Some(value) => value,
                            None => bail!("Missing field 'tlsClientAuth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#true_client_ip_header: {
                        let field_value = match fields_map.get("trueClientIpHeader") {
                            Some(value) => value,
                            None => bail!("Missing field 'trueClientIpHeader' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#universal_ssl: {
                        let field_value = match fields_map.get("universalSsl") {
                            Some(value) => value,
                            None => bail!("Missing field 'universalSsl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#visitor_ip: {
                        let field_value = match fields_map.get("visitorIp") {
                            Some(value) => value,
                            None => bail!("Missing field 'visitorIp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("zeroRtt") {
                            Some(value) => value,
                            None => bail!("Missing field 'zeroRtt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
