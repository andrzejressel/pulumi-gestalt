#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomHostnameSslSetting {
    /// List of SSL/TLS ciphers to associate with this certificate.
    #[builder(into)]
    #[serde(rename = "ciphers")]
    pub r#ciphers: Option<Vec<String>>,
    /// Whether early hints should be supported. Available values: `on`, `off`.
    #[builder(into)]
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Option<String>,
    /// Whether HTTP2 should be supported. Available values: `on`, `off`.
    #[builder(into)]
    #[serde(rename = "http2")]
    pub r#http_2: Option<String>,
    /// Lowest version of TLS this certificate should support. Available values: `1.0`, `1.1`, `1.2`, `1.3`.
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Option<String>,
    /// Whether TLSv1.3 should be supported. Available values: `on`, `off`.
    #[builder(into)]
    #[serde(rename = "tls13")]
    pub r#tls_13: Option<String>,
}
