#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZoneSettingsOverrideSettings {
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
    pub r#minify: Option<Box<super::types::ZoneSettingsOverrideSettingsMinify>>,
    #[builder(into)]
    #[serde(rename = "mirage")]
    pub r#mirage: Option<String>,
    #[builder(into)]
    #[serde(rename = "mobileRedirect")]
    pub r#mobile_redirect: Option<Box<super::types::ZoneSettingsOverrideSettingsMobileRedirect>>,
    #[builder(into)]
    #[serde(rename = "nel")]
    pub r#nel: Option<Box<super::types::ZoneSettingsOverrideSettingsNel>>,
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
    pub r#security_header: Option<Box<super::types::ZoneSettingsOverrideSettingsSecurityHeader>>,
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
