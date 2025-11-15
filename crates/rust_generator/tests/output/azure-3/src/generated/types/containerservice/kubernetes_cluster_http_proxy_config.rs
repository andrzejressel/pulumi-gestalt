#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterHttpProxyConfig {
    /// The proxy address to be used when communicating over HTTP.
    #[builder(into)]
    #[serde(rename = "httpProxy")]
    pub r#http_proxy: Option<String>,
    /// The proxy address to be used when communicating over HTTPS.
    #[builder(into)]
    #[serde(rename = "httpsProxy")]
    pub r#https_proxy: Option<String>,
    #[builder(into)]
    #[serde(rename = "noProxies")]
    pub r#no_proxies: Option<Vec<String>>,
    /// The base64 encoded alternative CA certificate content in PEM format.
    #[builder(into)]
    #[serde(rename = "trustedCa")]
    pub r#trusted_ca: Option<String>,
}
