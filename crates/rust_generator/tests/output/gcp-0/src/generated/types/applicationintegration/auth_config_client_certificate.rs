#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthConfigClientCertificate {
    /// The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines.
    #[builder(into)]
    #[serde(rename = "encryptedPrivateKey")]
    pub r#encrypted_private_key: String,
    /// 'passphrase' should be left unset if private key is not encrypted.
    /// Note that 'passphrase' is not the password for web server, but an extra layer of security to protected private key.
    #[builder(into)]
    #[serde(rename = "passphrase")]
    pub r#passphrase: Option<String>,
    /// The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines.
    #[builder(into)]
    #[serde(rename = "sslCertificate")]
    pub r#ssl_certificate: String,
}
