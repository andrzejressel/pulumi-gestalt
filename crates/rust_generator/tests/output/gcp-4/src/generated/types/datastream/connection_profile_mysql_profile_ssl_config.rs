#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionProfileMysqlProfileSslConfig {
    /// PEM-encoded certificate of the CA that signed the source database
    /// server's certificate.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "caCertificate")]
    pub r#ca_certificate: Option<String>,
    /// (Output)
    /// Indicates whether the clientKey field is set.
    #[builder(into)]
    #[serde(rename = "caCertificateSet")]
    pub r#ca_certificate_set: Option<bool>,
    /// PEM-encoded certificate that will be used by the replica to
    /// authenticate against the source database server. If this field
    /// is used then the 'clientKey' and the 'caCertificate' fields are
    /// mandatory.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Option<String>,
    /// (Output)
    /// Indicates whether the clientCertificate field is set.
    #[builder(into)]
    #[serde(rename = "clientCertificateSet")]
    pub r#client_certificate_set: Option<bool>,
    /// PEM-encoded private key associated with the Client Certificate.
    /// If this field is used then the 'client_certificate' and the
    /// 'ca_certificate' fields are mandatory.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Option<String>,
    /// (Output)
    /// Indicates whether the clientKey field is set.
    #[builder(into)]
    #[serde(rename = "clientKeySet")]
    pub r#client_key_set: Option<bool>,
}
