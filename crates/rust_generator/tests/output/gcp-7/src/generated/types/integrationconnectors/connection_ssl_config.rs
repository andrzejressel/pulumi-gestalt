#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionSslConfig {
    /// Additional SSL related field values.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalVariables")]
    pub r#additional_variables: Option<Vec<super::super::types::integrationconnectors::ConnectionSslConfigAdditionalVariable>>,
    /// Type of Client Cert (PEM/JKS/.. etc.)
    /// Possible values are: `PEM`.
    #[builder(into)]
    #[serde(rename = "clientCertType")]
    pub r#client_cert_type: Option<String>,
    /// Client Certificate
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigClientCertificate>>,
    /// Client Private Key
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clientPrivateKey")]
    pub r#client_private_key: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigClientPrivateKey>>,
    /// Secret containing the passphrase protecting the Client Private Key
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clientPrivateKeyPass")]
    pub r#client_private_key_pass: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigClientPrivateKeyPass>>,
    /// Private Server Certificate. Needs to be specified if trust model is PRIVATE.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "privateServerCertificate")]
    pub r#private_server_certificate: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigPrivateServerCertificate>>,
    /// Type of Server Cert (PEM/JKS/.. etc.)
    /// Possible values are: `PEM`.
    #[builder(into)]
    #[serde(rename = "serverCertType")]
    pub r#server_cert_type: Option<String>,
    /// Enum for Trust Model
    /// Possible values are: `PUBLIC`, `PRIVATE`, `INSECURE`.
    #[builder(into)]
    #[serde(rename = "trustModel")]
    pub r#trust_model: Option<String>,
    /// Enum for controlling the SSL Type (TLS/MTLS)
    /// Possible values are: `TLS`, `MTLS`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Bool for enabling SSL
    #[builder(into)]
    #[serde(rename = "useSsl")]
    pub r#use_ssl: Option<bool>,
}
