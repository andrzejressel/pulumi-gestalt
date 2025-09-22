#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeSourceParametersManagedStreamingKafkaParametersCredentials {
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into)]
    #[serde(rename = "clientCertificateTlsAuth")]
    pub r#client_certificate_tls_auth: Option<String>,
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into)]
    #[serde(rename = "saslScram512Auth")]
    pub r#sasl_scram_512_auth: Option<String>,
}
