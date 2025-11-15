#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetServerSSlInfo {
    /// The SSL/TLS cipher suites to be used. For programmable proxies, it must be one of the cipher suite names listed in: http://docs.oracle.com/javase/8/docs/technotes/guides/security/StandardNames.html#ciphersuites. For configurable proxies, it must follow the configuration specified in: https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#Cipher-suite-configuration. This setting has no effect for configurable proxies when negotiating TLS 1.3.
    #[builder(into)]
    #[serde(rename = "ciphers")]
    pub r#ciphers: Option<Vec<String>>,
    /// Enables two-way TLS.
    #[builder(into)]
    #[serde(rename = "clientAuthEnabled")]
    pub r#client_auth_enabled: Option<bool>,
    /// The TLS Common Name of the certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: Option<Box<super::super::types::apigee::TargetServerSSlInfoCommonName>>,
    /// Enables TLS. If false, neither one-way nor two-way TLS will be enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// If true, Edge ignores TLS certificate errors. Valid when configuring TLS for target servers and target endpoints, and when configuring virtual hosts that use 2-way TLS. When used with a target endpoint/target server, if the backend system uses SNI and returns a cert with a subject Distinguished Name (DN) that does not match the hostname, there is no way to ignore the error and the connection fails.
    #[builder(into)]
    #[serde(rename = "ignoreValidationErrors")]
    pub r#ignore_validation_errors: Option<bool>,
    /// Required if clientAuthEnabled is true. The resource ID for the alias containing the private key and cert.
    #[builder(into)]
    #[serde(rename = "keyAlias")]
    pub r#key_alias: Option<String>,
    /// Required if clientAuthEnabled is true. The resource ID of the keystore.
    #[builder(into)]
    #[serde(rename = "keyStore")]
    pub r#key_store: Option<String>,
    /// The TLS versioins to be used.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Option<Vec<String>>,
    /// The resource ID of the truststore.
    #[builder(into)]
    #[serde(rename = "trustStore")]
    pub r#trust_store: Option<String>,
}
