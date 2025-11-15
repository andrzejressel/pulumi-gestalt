#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewaySslProfile {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The name of the SSL Profile that is unique within this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// a `ssl_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "sslPolicy")]
    pub r#ssl_policy: Option<Box<super::super::types::network::ApplicationGatewaySslProfileSslPolicy>>,
    /// The name of the Trusted Client Certificate that will be used to authenticate requests from clients.
    #[builder(into)]
    #[serde(rename = "trustedClientCertificateNames")]
    pub r#trusted_client_certificate_names: Option<Vec<String>>,
    /// Should client certificate issuer DN be verified? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "verifyClientCertIssuerDn")]
    pub r#verify_client_cert_issuer_dn: Option<bool>,
    /// Specify the method to check client certificate revocation status. Possible value is `OCSP`.
    #[builder(into)]
    #[serde(rename = "verifyClientCertificateRevocation")]
    pub r#verify_client_certificate_revocation: Option<String>,
}
