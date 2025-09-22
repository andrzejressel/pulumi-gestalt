#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FluxConfigurationBlobStorageServicePrincipal {
    /// Base64-encoded certificate used to authenticate a Service Principal .
    #[builder(into)]
    #[serde(rename = "clientCertificateBase64")]
    pub r#client_certificate_base_64: Option<String>,
    /// Specifies the password for the certificate used to authenticate a Service Principal .
    #[builder(into)]
    #[serde(rename = "clientCertificatePassword")]
    pub r#client_certificate_password: Option<String>,
    /// Specifies whether to include x5c header in client claims when acquiring a token to enable subject name / issuer based authentication for the client certificate.
    #[builder(into)]
    #[serde(rename = "clientCertificateSendChain")]
    pub r#client_certificate_send_chain: Option<bool>,
    /// Specifies the client ID for authenticating a Service Principal.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// Specifies the client secret for authenticating a Service Principal.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// Specifies the tenant ID for authenticating a Service Principal.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
}
